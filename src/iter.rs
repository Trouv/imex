use crate::expression::{IMEx, IMExVal, QuantifiedIMExVal};
use std::{cell::RefCell, io::Result, rc::Rc, vec::IntoIter};

/// An iterator that lazily merges other iterators using an [`IMEx`](../expression/imex/struct.IMEx.html). The result of using the merge
/// functions defined on the [`IMExMerges`](../merges/trait.IMExMerges.html) trait.
pub struct IMExIter<T, I>
where
    T: Iterator<Item = I>,
{
    iters: Rc<RefCell<Vec<T>>>,
    imex: IntoIter<QuantifiedIMExVal>,
    current_imex_eval: (Option<IMExEval<T, I>>, bool),
    current_qimex_val: Option<QuantifiedIMExVal>,
}

enum IMExEval<T, I>
where
    T: Iterator<Item = I>,
{
    Single(usize),
    Group(Box<IMExIter<T, I>>),
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
    ///     .collect::<String>();
    ///
    /// assert_eq!(merged, "12a34bcde");
    /// ```
    pub fn from(iters: Vec<T>, imex: &str) -> Result<Self> {
        Ok(IMExIter::<T, I> {
            iters: Rc::new(RefCell::new(iters)),
            imex: IMEx::from(imex)?.0.into_iter(),
            current_imex_eval: (None, true),
            current_qimex_val: None,
        })
    }

    fn evaluate(&mut self) -> Option<I> {
        println!("1");
        let result = match &mut self.current_imex_eval.0 {
            Some(IMExEval::Single(i)) => {
                let res = match (*self.iters).borrow_mut().get_mut(*i) {
                    Some(e) => e.next(),
                    None => None,
                };
                println!("2");
                self.current_imex_eval.0 = None;
                res
            }
            Some(IMExEval::Group(b)) => match b.next() {
                Some(e) => Some(e),
                None => {
                    println!("3");
                    self.current_imex_eval.0 = None;
                    None
                }
            },
            None => None,
        };
        println!("4");

        if result.is_some() {
            self.current_imex_eval.1 = true;
        }
        result
    }

    fn repeat(&mut self) {
        println!("1");
        match &mut self.current_qimex_val {
            Some(q) if self.current_imex_eval.1 => match q.quantifier.next() {
                Some(_) => match &q.val {
                    IMExVal::Single(i) => {
                        println!("2");
                        self.current_imex_eval = (Some(IMExEval::Single(*i)), false)
                    }
                    IMExVal::Group(i) => {
                        println!("3");
                        self.current_imex_eval = (
                            Some(IMExEval::Group(Box::new(IMExIter {
                                iters: self.iters.clone(),
                                imex: i.0.clone().into_iter(),
                                current_imex_eval: (None, true),
                                current_qimex_val: None,
                            }))),
                            false,
                        )
                    }
                },
                None => self.current_qimex_val = None,
            },
            _ => self.current_qimex_val = None,
        }
    }
}

impl<T, I> Iterator for IMExIter<T, I>
where
    T: Iterator<Item = I>,
{
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            println!("{:?}", self.current_qimex_val);
            match self.evaluate() {
                Some(r) => return Some(r),
                None => {
                    self.repeat();
                    if self.current_qimex_val.is_none() {
                        self.current_qimex_val = match self.imex.next() {
                            Some(q) => Some(q),
                            None => return None,
                        };
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
    fn non_repeating_imex_might_not_complete() -> Result<()> {
        let iters = vec!["00000".chars(), "11111".chars()];
        let i = IMExIter::from(iters, "01(10){3}")?;

        assert_eq!(i.collect::<String>(), "01101010");

        Ok(())
    }

    #[test]
    fn repeating_imex_repeats() -> Result<()> {
        let iters = vec!["00000000".chars(), "111111".chars()];
        let i = IMExIter::from(iters, "0{3}1(01){5}")?;

        assert_eq!(i.collect::<String>(), "00010101010101");

        Ok(())
    }

    //#[test]
    fn completed_imex_exits_repeating() -> Result<()> {
        let iters = vec!["000".chars(), "111".chars(), "22222".chars()];
        let i = IMExIter::from(iters, "0*(12)*")?;

        assert_eq!(i.collect::<String>(), "00012121222");

        Ok(())
    }

    #[test]
    fn out_of_range_imex_skips() -> Result<()> {
        let iters = vec!["000".chars(), "111".chars()];
        let i = IMExIter::from(iters, "0120")?;

        assert_eq!(i.collect::<String>(), "010");

        Ok(())
    }

    #[test]
    fn empty_imex_gives_empty_merge() -> Result<()> {
        let iters = vec!["000".chars(), "111".chars()];
        let i = IMExIter::from(iters, "")?;

        assert_eq!(i.collect::<String>(), String::new());

        Ok(())
    }

    #[test]
    fn empty_iters_give_empty_merge() -> Result<()> {
        let iters = vec!["".chars(), "".chars(), "".chars()];
        let i = IMExIter::from(iters, "0120")?;

        assert_eq!(i.collect::<String>(), String::new());

        Ok(())
    }

    #[test]
    fn empty_iter_list_gives_empty_merge() -> Result<()> {
        let iters: Vec<std::str::Chars> = vec![];
        let i = IMExIter::from(iters, "0120")?;

        assert_eq!(i.collect::<String>(), String::new());

        Ok(())
    }
}
