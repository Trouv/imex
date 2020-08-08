use crate::zprex::{QuantifiedZprVal, ZprVal, Zprex};
use std::cell::RefCell;
use std::io::{Error, ErrorKind::InvalidInput, Result};
use std::rc::Rc;
use std::vec::IntoIter;

pub struct Zipper<T, I>
where
    T: Iterator<Item = I>,
{
    iters: Rc<RefCell<Vec<T>>>,
    zprex: IntoIter<QuantifiedZprVal>,
    inner_zipper: (Option<Box<Zipper<T, I>>>, bool),
    current_qzprval: Option<QuantifiedZprVal>,
}

impl<T, I> Zipper<T, I>
where
    T: Iterator<Item = I>,
{
    pub fn from(iters: Vec<T>, zprex: &str) -> Result<Self> {
        println!("{}", zprex);
        Ok(Zipper::<T, I> {
            iters: Rc::new(RefCell::new(iters)),
            zprex: Zprex::from(zprex)?.0.into_iter(),
            inner_zipper: (None, false),
            current_qzprval: None,
        })
    }
}

impl<T, I> Iterator for Zipper<T, I>
where
    T: Iterator<Item = I>,
{
    type Item = Result<I>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            println!("{:?}", self.current_qzprval);
            if let Some(z) = &mut self.inner_zipper.0 {
                let inner_res = z.next();

                if inner_res.is_some() {
                    self.inner_zipper.1 = true;
                    return inner_res;
                } else {
                    self.inner_zipper.0 = None;
                }
            }

            if let Some(q) = &mut self.current_qzprval {
                if q.quantifier.next().is_some() {
                    match &q.val {
                        ZprVal::Single(i) => {
                            if let Some(s) = (*self.iters).borrow_mut().get_mut(*i) {
                                if let Some(e) = s.next() {
                                    return Some(Ok(e));
                                } else {
                                    self.current_qzprval = None;
                                }
                            } else {
                                return Some(Err(Error::new(
                                    InvalidInput,
                                    format!("Zprex item out of file range: {}", i),
                                )));
                            }
                        }
                        ZprVal::Group(z) => {
                            if self.inner_zipper.1 {
                                self.inner_zipper = (
                                    Some(Box::from(Zipper::<T, I> {
                                        iters: self.iters.clone(),
                                        zprex: z.clone().0.into_iter(),
                                        inner_zipper: (None, false),
                                        current_qzprval: None,
                                    })),
                                    false,
                                );
                            } else {
                                self.current_qzprval = None;
                            }
                        }
                    }
                } else {
                    self.current_qzprval = None;
                }
            }

            if self.inner_zipper.0.is_none() && self.current_qzprval.is_none() {
                if let Some(q) = (self.zprex).next() {
                    self.current_qzprval = Some(q.clone());

                    if let ZprVal::Group(_) = q.val {
                        self.inner_zipper.1 = true;
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
    fn non_repeating_zprex_might_not_complete() -> Result<()> {
        let iters = vec!["00000".chars(), "11111".chars()];
        let z = Zipper::from(iters, "01(10){3}")?;

        assert_eq!(z.map(|c| c.unwrap()).collect::<String>(), "01101010");

        Ok(())
    }

    #[test]
    fn repeating_zprex_repeats() -> Result<()> {
        let iters = vec!["00000000".chars(), "111111".chars()];
        let z = Zipper::from(iters, "0{3}1(01){5}")?;

        assert_eq!(z.map(|c| c.unwrap()).collect::<String>(), "00010101010101");

        Ok(())
    }

    #[test]
    fn completed_zprex_exits_repeating() -> Result<()> {
        let iters = vec!["000".chars(), "111".chars(), "22222".chars()];
        let z = Zipper::from(iters, "0*(12)*")?;

        assert_eq!(z.map(|c| c.unwrap()).collect::<String>(), "00012121222");

        Ok(())
    }

    #[test]
    fn out_of_range_zprex_fails() -> Result<()> {
        let iters = vec!["000".chars(), "111".chars()];
        let mut z = Zipper::from(iters, "0120")?;

        if let Some(r) = z.nth(2) {
            r.unwrap_err();
        } else {
            panic!("Expected an error, not None");
        }

        Ok(())
    }

    #[test]
    fn empty_zprex_gives_empty_merge() -> Result<()> {
        let iters = vec!["000".chars(), "111".chars()];
        let z = Zipper::from(iters, "")?;

        assert_eq!(z.map(|c| c.unwrap()).collect::<String>(), String::new());

        Ok(())
    }

    #[test]
    fn empty_iters_give_empty_merge() -> Result<()> {
        let iters = vec!["".chars(), "".chars(), "".chars()];
        let z = Zipper::from(iters, "0120")?;

        assert_eq!(z.map(|c| c.unwrap()).collect::<String>(), String::new());

        Ok(())
    }

    #[test]
    fn empty_iter_list_only_passes_for_empty_zprex() -> Result<()> {
        let iters: Vec<std::str::Chars> = vec![];
        let mut z = Zipper::from(iters, "0120")?;

        if let Some(r) = z.nth(2) {
            r.unwrap_err();
        } else {
            panic!("Expected an error, not None");
        }

        let iters: Vec<std::str::Chars> = vec![];
        let z = Zipper::from(iters, "")?;

        assert_eq!(z.map(|c| c.unwrap()).collect::<String>(), String::new());

        Ok(())
    }

    // TODO: Tests for str::Lines vs io::Lines?
}
