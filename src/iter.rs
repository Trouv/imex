use crate::expression::{IMEx, IMExVal, QuantifiedIMExVal};
use std::{
    cell::RefCell,
    io::{Error, ErrorKind::InvalidInput, Result},
    rc::Rc,
    vec::IntoIter,
};

/// An iterator that lazily merges other iterators using an [`IMEx`](../imex/struct.IMEx.html). The result of using the merge
/// functions defined on the [`IMExMerges`](../merges/trait.IMExMerges.html) trait.
pub struct IMExIter<T, I>
where
    T: Iterator<Item = I>,
{
    iters: Rc<RefCell<Vec<T>>>,
    imex: IntoIter<QuantifiedIMExVal>,
    inner_imex_iter: (Option<Box<IMExIter<T, I>>>, bool),
    current_qimexval: Option<QuantifiedIMExVal>,
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
    /// let imex_iter = IMExIter::from(vec!["1234".chars(), "abcde".chars()], "(001)*")
    ///     .expect("Invalid IMEx");
    /// let merged = imex_iter
    ///     .map(|e| e.expect("Index out of range"))
    ///     .collect::<String>();
    ///
    /// assert_eq!(merged, "12a34bcde");
    /// ```
    pub fn from(iters: Vec<T>, imex: &str) -> Result<Self> {
        Ok(IMExIter::<T, I> {
            iters: Rc::new(RefCell::new(iters)),
            imex: IMEx::from(imex)?.0.into_iter(),
            inner_imex_iter: (None, false),
            current_qimexval: None,
        })
    }
}

impl<T, I> Iterator for IMExIter<T, I>
where
    T: Iterator<Item = I>,
{
    type Item = Result<I>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(i) = &mut self.inner_imex_iter.0 {
                let inner_res = i.next();

                if inner_res.is_some() {
                    self.inner_imex_iter.1 = true;
                    return inner_res;
                } else {
                    self.inner_imex_iter.0 = None;
                }
            }

            if let Some(q) = &mut self.current_qimexval {
                if q.quantifier.next().is_some() {
                    match &q.val {
                        IMExVal::Single(i) => {
                            if let Some(s) = (*self.iters).borrow_mut().get_mut(*i) {
                                if let Some(e) = s.next() {
                                    return Some(Ok(e));
                                } else {
                                    self.current_qimexval = None;
                                }
                            } else {
                                return Some(Err(Error::new(
                                    InvalidInput,
                                    format!("Zprex item out of file range: {}", i),
                                )));
                            }
                        }
                        IMExVal::Group(i) => {
                            if self.inner_imex_iter.1 {
                                self.inner_imex_iter = (
                                    Some(Box::from(IMExIter::<T, I> {
                                        iters: self.iters.clone(),
                                        imex: i.clone().0.into_iter(),
                                        inner_imex_iter: (None, false),
                                        current_qimexval: None,
                                    })),
                                    false,
                                );
                            } else {
                                self.current_qimexval = None;
                            }
                        }
                    }
                } else {
                    self.current_qimexval = None;
                }
            }

            if self.inner_imex_iter.0.is_none() && self.current_qimexval.is_none() {
                if let Some(q) = (self.imex).next() {
                    self.current_qimexval = Some(q.clone());

                    if let IMExVal::Group(_) = q.val {
                        self.inner_imex_iter.1 = true;
                    }
                } else {
                    return None;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_repeating_imex_might_not_complete() -> Result<()> {
        let iters = vec!["00000".chars(), "11111".chars()];
        let i = IMExIter::from(iters, "01(10){3}")?;

        assert_eq!(i.map(|c| c.unwrap()).collect::<String>(), "01101010");

        Ok(())
    }

    #[test]
    fn repeating_imex_repeats() -> Result<()> {
        let iters = vec!["00000000".chars(), "111111".chars()];
        let i = IMExIter::from(iters, "0{3}1(01){5}")?;

        assert_eq!(i.map(|c| c.unwrap()).collect::<String>(), "00010101010101");

        Ok(())
    }

    #[test]
    fn completed_imex_exits_repeating() -> Result<()> {
        let iters = vec!["000".chars(), "111".chars(), "22222".chars()];
        let i = IMExIter::from(iters, "0*(12)*")?;

        assert_eq!(i.map(|c| c.unwrap()).collect::<String>(), "00012121222");

        Ok(())
    }

    #[test]
    fn out_of_range_imex_fails() -> Result<()> {
        let iters = vec!["000".chars(), "111".chars()];
        let mut i = IMExIter::from(iters, "0120")?;

        if let Some(r) = i.nth(2) {
            r.unwrap_err();
        } else {
            panic!("Expected an error, not None");
        }

        Ok(())
    }

    #[test]
    fn empty_imex_gives_empty_merge() -> Result<()> {
        let iters = vec!["000".chars(), "111".chars()];
        let i = IMExIter::from(iters, "")?;

        assert_eq!(i.map(|c| c.unwrap()).collect::<String>(), String::new());

        Ok(())
    }

    #[test]
    fn empty_iters_give_empty_merge() -> Result<()> {
        let iters = vec!["".chars(), "".chars(), "".chars()];
        let i = IMExIter::from(iters, "0120")?;

        assert_eq!(i.map(|c| c.unwrap()).collect::<String>(), String::new());

        Ok(())
    }

    #[test]
    fn empty_iter_list_only_passes_for_empty_imex() -> Result<()> {
        let iters: Vec<std::str::Chars> = vec![];
        let mut i = IMExIter::from(iters, "0120")?;

        if let Some(r) = i.nth(2) {
            r.unwrap_err();
        } else {
            panic!("Expected an error, not None");
        }

        let iters: Vec<std::str::Chars> = vec![];
        let i = IMExIter::from(iters, "")?;

        assert_eq!(i.map(|c| c.unwrap()).collect::<String>(), String::new());

        Ok(())
    }
}
