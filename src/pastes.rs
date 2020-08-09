use crate::paste_iter::PasteIter;
use std::io::Result;

pub trait Pastes<T, I>
where
    T: Iterator<Item = I>,
{
    fn pastex(self, iters: &mut Vec<T>, pastex: &str) -> Result<PasteIter<T, I>>;

    fn paste(self, iters: &mut Vec<T>) -> PasteIter<T, I>
    where
        Self: Sized,
    {
        let iter_count = (iters.len() + 1) as u8;
        let pastex = format!(
            "({})*",
            (0..iter_count)
                .map(|x| format!("{}", x))
                .collect::<String>()
        );
        println!("{}", pastex);
        self.pastex(iters, &pastex)
            .expect("Default pastex should have been valid, but wasn't")
    }

    fn pastex_both(self, other: T, pastex: &str) -> Result<PasteIter<T, I>>
    where
        Self: Sized,
    {
        self.pastex(&mut vec![other], pastex)
    }

    fn paste_both(self, other: T) -> PasteIter<T, I>
    where
        Self: Sized,
    {
        self.pastex_both(other, "(01)*")
            .expect("Default pastex should have been valid, but wasn't")
    }
}

impl<T, I> Pastes<T, I> for T
where
    T: Iterator<Item = I>,
{
    fn pastex(self, iters: &mut Vec<T>, pastex: &str) -> Result<PasteIter<T, I>> {
        let mut total_iters = vec![self];
        total_iters.append(iters);
        PasteIter::<T, I>::from(total_iters, pastex)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pastex() -> Result<()> {
        let r = "12345"
            .chars()
            .pastex(&mut vec!["abcde".chars(), "!@#$%".chars()], "0(1120)*")?;

        assert_eq!(r.map(|i| i.unwrap()).collect::<String>(), "1ab!2cd@3e#4$5%");
        Ok(())
    }

    #[test]
    fn paste() {
        let r = "1234"
            .chars()
            .paste(&mut vec!["abcdefg".chars(), "!@#".chars()]);

        assert_eq!(r.map(|i| i.unwrap()).collect::<String>(), "1a!2b@3c#4defg");
    }

    #[test]
    fn pastex_both() -> Result<()> {
        let r = "12345".chars().pastex_both("ab".chars(), "0*1*")?;

        assert_eq!(r.map(|i| i.unwrap()).collect::<String>(), "12345ab");
        Ok(())
    }

    #[test]
    fn paste_both() {
        let r = "12".chars().paste_both("ab".chars());

        assert_eq!(r.map(|i| i.unwrap()).collect::<String>(), "1a2b");
    }
}
