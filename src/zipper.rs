use crate::zprex::Zprex;
use std::io::Result;

struct Zipper<I> {
    zprex: Zprex,
    iters: Vec<Box<dyn Iterator<Item = I>>>,
}

impl<I> Iterator for Zipper<I> {
    type Item = Result<I>;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_repeating_zprex_might_not_complete() -> Result<()> {
        let p = Zprex::from("01(10){3}")?;
        let mut iters = vec!["0\n0\n0\n0\n0".lines(), "1\n1\n1\n1\n1".lines()];

        assert_eq!(
            p.merge_iters(&mut iters)?,
            vec!["0", "1", "1", "0", "1", "0", "1", "0"]
        );

        Ok(())
    }

    #[test]
    fn repeating_zprex_repeats() -> Result<()> {
        let p = Zprex::from("0{3}1(01){5}")?;
        let mut iters = vec!["0\n0\n0\n0\n0\n0\n0\n0".lines(), "1\n1\n1\n1\n1\n1".lines()];

        assert_eq!(
            p.merge_iters(&mut iters)?,
            vec!["0", "0", "0", "1", "0", "1", "0", "1", "0", "1", "0", "1", "0", "1"]
        );

        Ok(())
    }

    #[test]
    fn completed_zprex_exits_repeating() -> Result<()> {
        let p = Zprex::from("0*(12)*")?;
        let mut iters = vec![
            "0\n0\n0".lines(),
            "1\n1\n1".lines(),
            "2\n2\n2\n2\n2".lines(),
        ];

        assert_eq!(
            p.merge_iters(&mut iters)?,
            vec!["0", "0", "0", "1", "2", "1", "2", "1", "2", "2", "2"]
        );

        Ok(())
    }

    #[test]
    fn out_of_range_zprex_fails() -> Result<()> {
        let p = Zprex::from("0120")?;
        let mut iters = vec!["0\n0\n0".lines(), "1\n1\n1".lines()];

        p.merge_iters(&mut iters).unwrap_err();

        Ok(())
    }

    #[test]
    fn empty_zprex_gives_empty_merge() -> Result<()> {
        let p = Zprex::from("")?;
        let mut iters = vec!["0\n0\n0".lines(), "1\n1\n1".lines()];

        assert_eq!(p.merge_iters(&mut iters)?, Vec::<String>::new());

        Ok(())
    }

    #[test]
    fn empty_iters_give_empty_merge() -> Result<()> {
        let p = Zprex::from("0120")?;
        let mut iters = vec!["".lines(), "".lines(), "".lines()];

        assert_eq!(p.merge_iters(&mut iters)?, Vec::<String>::new());

        Ok(())
    }

    #[test]
    fn empty_iter_list_only_passes_for_empty_zprex() -> Result<()> {
        let p = Zprex::from("0120")?;
        let mut iters: Vec<std::str::Lines> = vec![];

        p.merge_iters(&mut iters).unwrap_err();

        let p = Zprex::from("")?;

        assert_eq!(p.merge_iters(&mut iters)?, Vec::<String>::new());

        Ok(())
    }

    // TODO: Tests for str::Lines vs io::Lines?
}
