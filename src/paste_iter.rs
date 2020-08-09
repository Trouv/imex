use crate::pastex::{Pastex, PastexVal, QuantifiedPastexVal};
use std::cell::RefCell;
use std::io::{Error, ErrorKind::InvalidInput, Result};
use std::rc::Rc;
use std::vec::IntoIter;

pub struct PasteIter<T, I>
where
    T: Iterator<Item = I>,
{
    iters: Rc<RefCell<Vec<T>>>,
    pastex: IntoIter<QuantifiedPastexVal>,
    inner_paste_iter: (Option<Box<PasteIter<T, I>>>, bool),
    current_qpastexval: Option<QuantifiedPastexVal>,
}

impl<T, I> PasteIter<T, I>
where
    T: Iterator<Item = I>,
{
    pub fn from(iters: Vec<T>, pastex: &str) -> Result<Self> {
        println!("{}", pastex);
        Ok(PasteIter::<T, I> {
            iters: Rc::new(RefCell::new(iters)),
            pastex: Pastex::from(pastex)?.0.into_iter(),
            inner_paste_iter: (None, false),
            current_qpastexval: None,
        })
    }
}

impl<T, I> Iterator for PasteIter<T, I>
where
    T: Iterator<Item = I>,
{
    type Item = Result<I>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            println!("{:?}", self.current_qpastexval);
            if let Some(p) = &mut self.inner_paste_iter.0 {
                let inner_res = p.next();

                if inner_res.is_some() {
                    self.inner_paste_iter.1 = true;
                    return inner_res;
                } else {
                    self.inner_paste_iter.0 = None;
                }
            }

            if let Some(q) = &mut self.current_qpastexval {
                if q.quantifier.next().is_some() {
                    match &q.val {
                        PastexVal::Single(i) => {
                            if let Some(s) = (*self.iters).borrow_mut().get_mut(*i) {
                                if let Some(e) = s.next() {
                                    return Some(Ok(e));
                                } else {
                                    self.current_qpastexval = None;
                                }
                            } else {
                                return Some(Err(Error::new(
                                    InvalidInput,
                                    format!("Zprex item out of file range: {}", i),
                                )));
                            }
                        }
                        PastexVal::Group(p) => {
                            if self.inner_paste_iter.1 {
                                self.inner_paste_iter = (
                                    Some(Box::from(PasteIter::<T, I> {
                                        iters: self.iters.clone(),
                                        pastex: p.clone().0.into_iter(),
                                        inner_paste_iter: (None, false),
                                        current_qpastexval: None,
                                    })),
                                    false,
                                );
                            } else {
                                self.current_qpastexval = None;
                            }
                        }
                    }
                } else {
                    self.current_qpastexval = None;
                }
            }

            if self.inner_paste_iter.0.is_none() && self.current_qpastexval.is_none() {
                if let Some(q) = (self.pastex).next() {
                    self.current_qpastexval = Some(q.clone());

                    if let PastexVal::Group(_) = q.val {
                        self.inner_paste_iter.1 = true;
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
    fn non_repeating_pastex_might_not_complete() -> Result<()> {
        let iters = vec!["00000".chars(), "11111".chars()];
        let p = PasteIter::from(iters, "01(10){3}")?;

        assert_eq!(p.map(|c| c.unwrap()).collect::<String>(), "01101010");

        Ok(())
    }

    #[test]
    fn repeating_pastex_repeats() -> Result<()> {
        let iters = vec!["00000000".chars(), "111111".chars()];
        let p = PasteIter::from(iters, "0{3}1(01){5}")?;

        assert_eq!(p.map(|c| c.unwrap()).collect::<String>(), "00010101010101");

        Ok(())
    }

    #[test]
    fn completed_pastex_exits_repeating() -> Result<()> {
        let iters = vec!["000".chars(), "111".chars(), "22222".chars()];
        let p = PasteIter::from(iters, "0*(12)*")?;

        assert_eq!(p.map(|c| c.unwrap()).collect::<String>(), "00012121222");

        Ok(())
    }

    #[test]
    fn out_of_range_pastex_fails() -> Result<()> {
        let iters = vec!["000".chars(), "111".chars()];
        let mut p = PasteIter::from(iters, "0120")?;

        if let Some(r) = p.nth(2) {
            r.unwrap_err();
        } else {
            panic!("Expected an error, not None");
        }

        Ok(())
    }

    #[test]
    fn empty_pastex_gives_empty_merge() -> Result<()> {
        let iters = vec!["000".chars(), "111".chars()];
        let p = PasteIter::from(iters, "")?;

        assert_eq!(p.map(|c| c.unwrap()).collect::<String>(), String::new());

        Ok(())
    }

    #[test]
    fn empty_iters_give_empty_merge() -> Result<()> {
        let iters = vec!["".chars(), "".chars(), "".chars()];
        let p = PasteIter::from(iters, "0120")?;

        assert_eq!(p.map(|c| c.unwrap()).collect::<String>(), String::new());

        Ok(())
    }

    #[test]
    fn empty_iter_list_only_passes_for_empty_pastex() -> Result<()> {
        let iters: Vec<std::str::Chars> = vec![];
        let mut p = PasteIter::from(iters, "0120")?;

        if let Some(r) = p.nth(2) {
            r.unwrap_err();
        } else {
            panic!("Expected an error, not None");
        }

        let iters: Vec<std::str::Chars> = vec![];
        let p = PasteIter::from(iters, "")?;

        assert_eq!(p.map(|c| c.unwrap()).collect::<String>(), String::new());

        Ok(())
    }

    // TODO: Tests for str::Lines vs io::Lines?
}
