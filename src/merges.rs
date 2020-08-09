use crate::iter::IMExIter;
use std::io::Result;

pub trait IMExMerges<T, I>
where
    T: Iterator<Item = I>,
{
    fn imex_merge_all(self, iters: &mut Vec<T>, imex: &str) -> Result<IMExIter<T, I>>;

    fn rot_merge_all(self, iters: &mut Vec<T>) -> IMExIter<T, I>
    where
        Self: Sized,
    {
        let iter_count = (iters.len() + 1) as u8;
        let imex = format!(
            "({})*",
            (0..iter_count)
                .map(|x| format!("{}", x))
                .collect::<String>()
        );
        println!("{}", imex);
        self.imex_merge_all(iters, &imex)
            .expect("Default imex should have been valid, but wasn't")
    }

    fn imex_merge(self, other: T, imex: &str) -> Result<IMExIter<T, I>>
    where
        Self: Sized,
    {
        self.imex_merge_all(&mut vec![other], imex)
    }

    fn alt_merge(self, other: T) -> IMExIter<T, I>
    where
        Self: Sized,
    {
        self.imex_merge(other, "(01)*")
            .expect("Default imex should have been valid, but wasn't")
    }
}

impl<T, I> IMExMerges<T, I> for T
where
    T: Iterator<Item = I>,
{
    fn imex_merge_all(self, iters: &mut Vec<T>, imex: &str) -> Result<IMExIter<T, I>> {
        let mut total_iters = vec![self];
        total_iters.append(iters);
        IMExIter::<T, I>::from(total_iters, imex)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imex_merge_all() -> Result<()> {
        let r = "12345"
            .chars()
            .imex_merge_all(&mut vec!["abcde".chars(), "!@#$%".chars()], "0(1120)*")?;

        assert_eq!(r.map(|i| i.unwrap()).collect::<String>(), "1ab!2cd@3e#4$5%");
        Ok(())
    }

    #[test]
    fn rot_merge_all() {
        let r = "1234"
            .chars()
            .rot_merge_all(&mut vec!["abcdefg".chars(), "!@#".chars()]);

        assert_eq!(r.map(|i| i.unwrap()).collect::<String>(), "1a!2b@3c#4defg");
    }

    #[test]
    fn imex_merge() -> Result<()> {
        let r = "12345".chars().imex_merge("ab".chars(), "0*1*")?;

        assert_eq!(r.map(|i| i.unwrap()).collect::<String>(), "12345ab");
        Ok(())
    }

    #[test]
    fn alt_merge() {
        let r = "12".chars().alt_merge("ab".chars());

        assert_eq!(r.map(|i| i.unwrap()).collect::<String>(), "1a2b");
    }
}
