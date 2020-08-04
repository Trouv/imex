use crate::repeater::Repeater;
use crate::zprex::{QuantifiedZprVal, ZprVal, Zprex};
use std::io::{Error, ErrorKind::InvalidInput, Result};

struct Zipper<I> {
    zprex: Box<dyn Iterator<Item = QuantifiedZprVal>>,
    iters: Vec<Box<dyn Iterator<Item = I>>>,
    inner_zipper: Option<Box<Zipper<I>>>,
    current_repeater: Option<Box<Repeater<I>>>,
}

impl<I> Zipper<I> {
    fn from(zprex: &str, iters: Vec<Box<dyn Iterator<Item = I>>>) -> Result<Self> {
        Ok(Zipper::<I> {
            zprex: Box::from(Zprex::from(zprex)?.0.into_iter()),
            iters,
            inner_zipper: None,
            current_repeater: None,
        })
    }
}

impl<I> Iterator for Zipper<I> {
    type Item = Result<I>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(z) = self.inner_zipper {
                let inner_res = (*z).next();

                if inner_res.is_some() {
                    return inner_res;
                } else {
                    self.inner_zipper = None;
                }
            }

            if let Some(r) = self.current_repeater {
                let rep_res = (*r).next();

                if rep_res.is_some() {
                    return rep_res;
                } else {
                    self.current_repeater = None;
                }
            }

            if let Some(q) = (*self.zprex).next() {
                match q.val {
                    ZprVal::Single(i) => {
                        self.current_repeater = Repeater {
                            quantifier: q.quantifier,
                            operation: || -> Option<Self::Item> {
                                if let Some(s) = self.iters.get_mut(*i) {
                                    if let Some(e) = s.next() {
                                        Some(Ok(e))
                                    } else {
                                        None
                                    }
                                } else {
                                    Err(Error::new(
                                        InvalidInput,
                                        format!("Zprex item out of file range: {}", i),
                                    ))
                                }
                            },
                        }
                    }
                    ZprVal::Group(z) => {
                        self.current_repeater = Repeater {
                            quantifier: q.quantifier,
                            operatiion: || -> Option<Self::Item> { self.inner_zprex = z },
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_repeating_zprex_might_not_complete() -> Result<()> {
        let iters: Vec<Box<dyn Iterator<Item = char>>> =
            vec![Box::from("00000".chars()), Box::from("11111".chars())];
        let z = Zipper::from("01(10){3}", iters)?;

        assert_eq!(z.map(|c| c.unwrap()).collect::<String>(), "01010");

        Ok(())
    }

    #[test]
    fn repeating_zprex_repeats() -> Result<()> {
        let iters: Vec<Box<dyn Iterator<Item = char>>> =
            vec![Box::from("00000000".chars()), Box::from("111111".chars())];
        let z = Zipper::from("0{3}1(01){5}", iters)?;

        assert_eq!(z.map(|c| c.unwrap()).collect::<String>(), "00010101010101");

        Ok(())
    }

    #[test]
    fn completed_zprex_exits_repeating() -> Result<()> {
        let iters: Vec<Box<dyn Iterator<Item = char>>> = vec![
            Box::from("000".chars()),
            Box::from("111".chars()),
            Box::from("22222".chars()),
        ];
        let z = Zipper::from("0*(12)*", iters)?;

        assert_eq!(z.map(|c| c.unwrap()).collect::<String>(), "00012121222");

        Ok(())
    }

    #[test]
    fn out_of_range_zprex_fails() -> Result<()> {
        let iters: Vec<Box<dyn Iterator<Item = char>>> =
            vec![Box::from("000".chars()), Box::from("111".chars())];
        let mut z = Zipper::from("0120", iters)?;

        if let Some(r) = z.nth(2) {
            r.unwrap_err();
        } else {
            panic!("Expected an error, not None");
        }

        Ok(())
    }

    #[test]
    fn empty_zprex_gives_empty_merge() -> Result<()> {
        let iters: Vec<Box<dyn Iterator<Item = char>>> =
            vec![Box::from("000".chars()), Box::from("111".chars())];
        let z = Zipper::from("", iters)?;

        assert_eq!(z.map(|c| c.unwrap()).collect::<String>(), String::new());

        Ok(())
    }

    #[test]
    fn empty_iters_give_empty_merge() -> Result<()> {
        let iters: Vec<Box<dyn Iterator<Item = char>>> = vec![
            Box::from("".chars()),
            Box::from("".chars()),
            Box::from("".chars()),
        ];
        let z = Zipper::from("0120", iters)?;

        assert_eq!(z.map(|c| c.unwrap()).collect::<String>(), String::new());

        Ok(())
    }

    #[test]
    fn empty_iter_list_only_passes_for_empty_zprex() -> Result<()> {
        let iters: Vec<Box<dyn Iterator<Item = char>>> = vec![];
        let mut z = Zipper::from("0120", iters)?;

        if let Some(r) = z.nth(2) {
            r.unwrap_err();
        } else {
            panic!("Expected an error, not None");
        }

        let iters: Vec<Box<dyn Iterator<Item = char>>> = vec![];
        let z = Zipper::from("", iters)?;

        assert_eq!(z.map(|c| c.unwrap()).collect::<String>(), String::new());

        Ok(())
    }

    // TODO: Tests for str::Lines vs io::Lines?
}
