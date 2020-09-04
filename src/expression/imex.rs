use super::{parsers::parse_imex, IMExIterator, QuantifiedIMExVal};
use std::{
    io::{Error, ErrorKind::InvalidInput, Result},
    vec::IntoIter,
};

/// A single-element tuple-struct representing a parsed [`IMEx`](./struct.IMEx.html). Used by
/// [`IMExIter`](../../merges/trait.IMExMerges.html) to perform lazy merging.
#[derive(Debug, Clone)]
pub struct IMEx {
    pub vals: IntoIter<QuantifiedIMExVal>,
    current_val: Option<Box<QuantifiedIMExVal>>,
}

impl PartialEq for IMEx {
    fn eq(&self, other: &IMEx) -> bool {
        self.vals.clone().collect::<Vec<QuantifiedIMExVal>>()
            == other.vals.clone().collect::<Vec<QuantifiedIMExVal>>()
    }
}

impl IMEx {
    /// Parse an [`IMEx`](./struct.IMEx.html) from a string.
    ///
    /// # Error
    /// Results in an error if the IMEx is invalid.
    ///
    /// # Example
    /// ```
    /// use imex::expression::IMEx;
    /// let imex = IMEx::from("01*(23){4}");
    /// ```
    pub fn from(imex_str: &str) -> Result<Self> {
        match parse_imex(imex_str) {
            Ok((_, imex)) => Ok(imex),
            Err(e) => Err(Error::new(InvalidInput, format!("{}", e))),
        }
    }

    pub fn new(vals: IntoIter<QuantifiedIMExVal>) -> IMEx {
        IMEx {
            vals,
            current_val: None,
        }
    }
}

impl<T, I> IMExIterator<T, I> for IMEx
where
    T: Iterator<Item = I>,
{
    fn iterate(&mut self, iters: &mut Vec<T>) -> Option<I> {
        loop {
            match &mut self.current_val {
                Some(val) => match val.iterate(iters) {
                    Some(res) => return Some(res),
                    None => self.current_val = None,
                },
                None => match self.vals.next() {
                    Some(val) => self.current_val = Some(Box::new(val)),
                    None => return None,
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::{IMExVal, Quantifier};
    use super::*;
    use std::iter::once;

    #[test]
    fn empty_string_gives_empty_group_imex() -> Result<()> {
        let i = IMEx::from("")?;

        assert_eq!(i, IMEx::new(vec![].into_iter()));
        Ok(())
    }

    #[test]
    fn repeats_gives_repeating_imex() -> Result<()> {
        let i = IMEx::from("13{3}9*1")?;

        assert_eq!(
            i,
            IMEx::new(
                vec![
                    QuantifiedIMExVal::from(IMExVal::Single(once(1)), Quantifier::Finite(1),),
                    QuantifiedIMExVal::from(IMExVal::Single(once(3)), Quantifier::Finite(3),),
                    QuantifiedIMExVal::from(IMExVal::Single(once(9)), Quantifier::Infinite,),
                    QuantifiedIMExVal::from(IMExVal::Single(once(1)), Quantifier::Finite(1),),
                ]
                .into_iter(),
            )
        );
        Ok(())
    }

    #[test]
    fn parens_gives_group_imex() -> Result<()> {
        let i = IMEx::from("1(1)(9)*(4){45}(1(1))()")?;

        assert_eq!(
            i,
            IMEx::new(
                vec![
                    QuantifiedIMExVal::from(IMExVal::Single(once(1)), Quantifier::Finite(1),),
                    QuantifiedIMExVal::from(
                        IMExVal::Group(IMEx::new(
                            vec![QuantifiedIMExVal::from(
                                IMExVal::Single(once(1)),
                                Quantifier::Finite(1),
                            )]
                            .into_iter()
                        )),
                        Quantifier::Finite(1),
                    ),
                    QuantifiedIMExVal::from(
                        IMExVal::Group(IMEx::new(
                            vec![QuantifiedIMExVal::from(
                                IMExVal::Single(once(9)),
                                Quantifier::Finite(1),
                            )]
                            .into_iter()
                        )),
                        Quantifier::Infinite,
                    ),
                    QuantifiedIMExVal::from(
                        IMExVal::Group(IMEx::new(
                            vec![QuantifiedIMExVal::from(
                                IMExVal::Single(once(4)),
                                Quantifier::Finite(1),
                            )]
                            .into_iter()
                        )),
                        Quantifier::Finite(45),
                    ),
                    QuantifiedIMExVal::from(
                        IMExVal::Group(IMEx::new(
                            vec![
                                QuantifiedIMExVal::from(
                                    IMExVal::Single(once(1)),
                                    Quantifier::Finite(1),
                                ),
                                QuantifiedIMExVal::from(
                                    IMExVal::Group(IMEx::new(
                                        vec![QuantifiedIMExVal::from(
                                            IMExVal::Single(once(1)),
                                            Quantifier::Finite(1),
                                        )]
                                        .into_iter()
                                    )),
                                    Quantifier::Finite(1),
                                )
                            ]
                            .into_iter()
                        )),
                        Quantifier::Finite(1),
                    ),
                    QuantifiedIMExVal::from(
                        IMExVal::Group(IMEx::new(vec![].into_iter())),
                        Quantifier::Finite(1),
                    )
                ]
                .into_iter()
            )
        );
        Ok(())
    }

    #[test]
    fn bad_chars_fails() {
        IMEx::from("0O0").unwrap_err();

        IMEx::from("^[0]+$").unwrap_err();

        IMEx::from("123*4{5}(x)*").unwrap_err();
    }

    #[test]
    fn too_many_closed_parens_fails() {
        IMEx::from("0(1)2)3(4)5").unwrap_err();

        IMEx::from("0{1}2}3{4}5").unwrap_err();

        IMEx::from("0(1)2(3))").unwrap_err();
    }

    #[test]
    fn too_many_open_parens_fails() {
        IMEx::from("0(1)2(3(4)5").unwrap_err();

        IMEx::from("0{1}2{3{4}5").unwrap_err();

        IMEx::from("0{1}23{4}5{6").unwrap_err();

        IMEx::from("((0)1(2)3").unwrap_err();
    }

    #[test]
    fn mismatched_parens_fails() {
        IMEx::from(")(").unwrap_err();

        IMEx::from("(3{)}").unwrap_err();
    }

    #[test]
    fn bad_repeat_targets_fails() {
        IMEx::from("(*4)").unwrap_err();

        IMEx::from("({6}6)").unwrap_err();

        IMEx::from("*2").unwrap_err();

        IMEx::from("{4}4").unwrap_err();

        IMEx::from("5{*5}").unwrap_err();
    }

    #[test]
    fn bad_repeat_bracket_contents_fails() {
        IMEx::from("5{5*}").unwrap_err();

        IMEx::from("6{(6)}").unwrap_err();

        IMEx::from("7{7{7}}").unwrap_err();
    }
}
