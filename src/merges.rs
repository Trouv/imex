use crate::IMExIter;
use std::io::Result;

/// Trait for merging iterators into an [`IMExIter`](../iter/struct.IMExIter.html)
pub trait IMExMerges<T, I>
where
    T: Iterator<Item = I>,
{
    /// Define how self, a vector of iterators, and an IMEx string merge into an
    /// [`IMExIter`](../iter/struct.IMExIter.html)
    ///
    /// In practice, this is used to merge more than two iterators with a custom IMEx.
    ///
    /// # Error
    /// Results in an error if the provided IMEx is invalid.
    ///
    /// # Example
    /// ```
    /// use imex::IMExMerges;
    ///
    /// let merged = "12345"
    ///     .chars()
    ///     .imex_merge_all(&mut vec!["abcde".chars(), "!@#$%".chars()], "0(1120)*")
    ///     .expect("Invalid IMEx")
    ///     .collect::<String>();
    ///
    /// assert_eq!(merged, "1ab!2cd@3e#4$5%");
    /// ```
    fn imex_merge_all(self, iters: &mut Vec<T>, imex: &str) -> Result<IMExIter<T, I>>;

    /// Merges many iterators using a rotating IMEx. The resulting iterator will consume 1 item
    /// from the provided iterators, in order, until they are all exhausted.
    ///
    /// # Example
    /// ```
    /// use imex::IMExMerges;
    ///
    /// let merged = "1234"
    ///     .chars()
    ///     .rot_merge_all(&mut vec!["abcdefg".chars(), "!@#".chars()])
    ///     .collect::<String>();
    ///
    /// assert_eq!(merged, "1a!2b@3c#4defg");
    /// ```
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
        self.imex_merge_all(iters, &imex)
            .expect("Default imex should have been valid, but wasn't")
    }

    /// Merges two iterators (self and other) using a custom IMEx.
    ///
    /// # Error
    /// Results in an error if the provided IMEx is invalid.
    ///
    /// # Example
    /// ```
    /// use imex::IMExMerges;
    ///
    /// let merged = "12345"
    ///     .chars()
    ///     .imex_merge("ab".chars(), "0*1*")
    ///     .expect("Invalid IMEx")
    ///     .collect::<String>();
    ///
    /// assert_eq!(merged, "12345ab");
    /// ```
    fn imex_merge(self, other: T, imex: &str) -> Result<IMExIter<T, I>>
    where
        Self: Sized,
    {
        self.imex_merge_all(&mut vec![other], imex)
    }

    /// Merges two iterators (self and other) using an alternating IMEx. The resulting iterator
    /// will consume 1 item from self and other, alternating, until they are both exhausted.
    ///
    /// # Example
    /// ```
    /// use imex::IMExMerges;
    ///
    /// let merged = "12"
    ///     .chars()
    ///     .alt_merge("ab".chars())
    ///     .collect::<String>();
    ///
    /// assert_eq!(merged, "1a2b");
    /// ```
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
        IMExIter::<T, I>::new(total_iters, imex)
    }
}
