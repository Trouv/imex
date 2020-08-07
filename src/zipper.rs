use crate::zprex::{QuantifiedZprVal, ZprVal, Zprex};
use std::cell::RefCell;
use std::io::{Error, ErrorKind::InvalidInput, Result};
use std::rc::Rc;

pub struct Zipper<I> {
    iters: Rc<RefCell<Vec<Box<dyn Iterator<Item = I>>>>>,
    zprex: Box<dyn Iterator<Item = QuantifiedZprVal>>,
    inner_zipper: (Option<Box<Zipper<I>>>, bool),
    current_qzprval: Option<QuantifiedZprVal>,
}

impl<I> Zipper<I> {
    pub fn from(iters: Vec<Box<dyn Iterator<Item = I>>>, zprex: &str) -> Result<Self> {
        println!("{}", zprex);
        Ok(Zipper::<I> {
            iters: Rc::new(RefCell::from(iters)),
            zprex: Box::from(Zprex::from(zprex)?.0.into_iter()),
            inner_zipper: (None, false),
            current_qzprval: None,
        })
    }
}

impl<I> Iterator for Zipper<I> {
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
                                    Some(Box::from(Zipper {
                                        zprex: Box::from(z.0.clone().into_iter()),
                                        iters: self.iters.clone(),
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
                if let Some(q) = (*self.zprex).next() {
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
        let iters: Vec<Box<dyn Iterator<Item = char>>> =
            vec![Box::from("00000".chars()), Box::from("11111".chars())];
        let z = Zipper::from("01(10){3}", iters)?;

        assert_eq!(z.map(|c| c.unwrap()).collect::<String>(), "01101010");

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
