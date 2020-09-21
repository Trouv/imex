use crate::expression::IMEx;
use std::convert::TryFrom;
use std::io::Result;

/// Similar to the standard Iterator, except its iteration function requires an extra argument.
///
/// Used on many of the objects in the expression module.
pub trait IMExIterator {
    /// Defines how the IMExIterator iterates and returns a result using the iters that an IMEx is
    /// supposed to merge.
    fn iterate<T, I>(&mut self, iters: &mut Vec<T>) -> Option<I>
    where
        T: Iterator<Item = I>;
}

/// An iterator that lazily merges other iterators using an
/// [`IMEx`](./expression/imex/struct.IMEx.html). The result of using the merge functions defined
/// on the [`IMExMerges`](./trait.IMExMerges.html) trait.
pub struct IMExIter<T, I>
where
    T: Iterator<Item = I>,
{
    iters: Vec<T>,
    imex: IMEx,
}

impl<T, I> IMExIter<T, I>
where
    T: Iterator<Item = I>,
{
    /// Constructs and [`IMExIter`](./struct.IMExIter.html) from a vector of iterators and an IMEx
    /// string.
    ///
    /// # Error
    /// Results in an error if the provided IMEx is invalid.
    ///
    /// # Example
    /// ```
    /// use imex::IMExIter;
    ///
    /// let imex_iter = IMExIter::new(vec!["1234".chars(), "abcde".chars()], "(001)*")
    ///     .expect("Invalid IMEx");
    /// let merged = imex_iter
    ///     .collect::<String>();
    ///
    /// assert_eq!(merged, "12a34bcde");
    /// ```
    pub fn new(iters: Vec<T>, imex: &str) -> Result<Self> {
        Ok(IMExIter::<T, I> {
            iters,
            imex: IMEx::try_from(imex)?,
        })
    }
}

impl<T, I> Iterator for IMExIter<T, I>
where
    T: Iterator<Item = I>,
{
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        self.imex.iterate(&mut self.iters)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_repeating_imex_might_not_complete() -> Result<()> {
        let iters = vec!["00000".chars(), "11111".chars()];
        let i = IMExIter::new(iters, "01(10){3}")?;

        assert_eq!(i.collect::<String>(), "01101010");

        Ok(())
    }

    #[test]
    fn repeating_imex_repeats() -> Result<()> {
        let iters = vec!["00000000".chars(), "111111".chars()];
        let i = IMExIter::new(iters, "0{3}1(01){5}")?;

        assert_eq!(i.collect::<String>(), "00010101010101");

        Ok(())
    }

    #[test]
    fn completed_imex_exits_repeating() -> Result<()> {
        let iters = vec!["000".chars(), "111".chars(), "22222".chars()];
        let i = IMExIter::new(iters, "0*(12)*")?;

        assert_eq!(i.collect::<String>(), "00012121222");

        Ok(())
    }

    #[test]
    fn out_of_range_imex_skips() -> Result<()> {
        let iters = vec!["000".chars(), "111".chars()];
        let i = IMExIter::new(iters, "0120")?;

        assert_eq!(i.collect::<String>(), "010");

        Ok(())
    }

    #[test]
    fn empty_imex_gives_empty_merge() -> Result<()> {
        let iters = vec!["000".chars(), "111".chars()];
        let i = IMExIter::new(iters, "")?;

        assert_eq!(i.collect::<String>(), String::new());

        Ok(())
    }

    #[test]
    fn empty_iters_give_empty_merge() -> Result<()> {
        let iters = vec!["".chars(), "".chars(), "".chars()];
        let i = IMExIter::new(iters, "0120")?;

        assert_eq!(i.collect::<String>(), String::new());

        Ok(())
    }

    #[test]
    fn empty_iter_list_gives_empty_merge() -> Result<()> {
        let iters: Vec<std::str::Chars> = vec![];
        let i = IMExIter::new(iters, "0120")?;

        assert_eq!(i.collect::<String>(), String::new());

        Ok(())
    }
}
