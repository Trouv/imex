use crate::{
    expression::{ParserCombinator, QuantifiedIMExVal},
    IMExIterator,
};
use nom::{
    character::complete::char,
    combinator::all_consuming,
    combinator::complete,
    error::{convert_error, VerboseError},
    multi::{many0, many_till},
    IResult,
};
use std::{
    convert::TryFrom,
    io::{Error, ErrorKind::InvalidInput, Result},
    vec::IntoIter,
};

/// A single-element tuple-struct representing a parsed [`IMEx`](./struct.IMEx.html). Used by
/// [`IMExIter`](../struct.IMExIter.html) to perform lazy merging.
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
    fn new(vals: IntoIter<QuantifiedIMExVal>) -> IMEx {
        IMEx {
            vals,
            current_val: None,
        }
    }

    /// Parser combinator for parsing an [`IMEx`](./struct.IMEx.html), making use of the
    /// nom library.
    ///
    /// Currently, this parser combinator expects to be "all consuming", which means it will fail if
    /// there is any input string remaining after parsing an IMEx.
    ///
    /// # Error
    /// Results in an error if the input string is not a valid IMEx.
    fn parse_complete(input: &str) -> IResult<&str, IMEx, VerboseError<&str>> {
        let (input, imex) = complete(all_consuming(many0(QuantifiedIMExVal::parse)))(input)?;
        Ok((input, IMEx::new(imex.into_iter())))
    }
}

impl TryFrom<&str> for IMEx {
    type Error = Error;

    /// Parse an [`IMEx`](./struct.IMEx.html) from a string.
    ///
    /// # Error
    /// Results in an error if the IMEx is invalid.
    ///
    /// # Example
    /// ```
    /// use imex::expression::IMEx;
    /// use std::convert::TryFrom;
    /// let imex = IMEx::try_from("01*(23){4}");
    /// ```
    fn try_from(imex_str: &str) -> Result<Self> {
        match IMEx::parse_complete(imex_str) {
            Ok((_, imex)) => Ok(imex),
            Err(e) => Err(Error::new(
                InvalidInput,
                convert_error(
                    imex_str,
                    match e {
                        nom::Err::Error(verbose) => verbose,
                        nom::Err::Failure(verbose) => verbose,
                        _ => panic!("Expected input to be complete"),
                    },
                ),
            )),
        }
    }
}

impl IMExIterator for IMEx {
    fn iterate<T, I>(&mut self, iters: &mut Vec<T>) -> Option<I>
    where
        T: Iterator<Item = I>,
    {
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

impl ParserCombinator for IMEx {
    fn parse(input: &str) -> IResult<&str, IMEx, VerboseError<&str>> {
        let (input, imex) = many_till(QuantifiedIMExVal::parse, char(')'))(input)?;
        Ok((input, IMEx::new(imex.0.into_iter())))
    }
}

#[cfg(test)]
mod tests {
    use super::super::{IMExVal, Quantifier};
    use super::*;
    use std::iter::once;

    #[test]
    fn empty_string_gives_empty_group_imex() -> Result<()> {
        let i = IMEx::try_from("")?;

        assert_eq!(i, IMEx::new(vec![].into_iter()));
        Ok(())
    }

    #[test]
    fn repeats_gives_repeating_imex() -> Result<()> {
        let i = IMEx::try_from("13{3}9*1")?;

        assert_eq!(
            i,
            IMEx::new(
                vec![
                    QuantifiedIMExVal::new(IMExVal::Single(once(1)), Quantifier::Finite(1),),
                    QuantifiedIMExVal::new(IMExVal::Single(once(3)), Quantifier::Finite(3),),
                    QuantifiedIMExVal::new(IMExVal::Single(once(9)), Quantifier::Infinite,),
                    QuantifiedIMExVal::new(IMExVal::Single(once(1)), Quantifier::Finite(1),),
                ]
                .into_iter(),
            )
        );
        Ok(())
    }

    #[test]
    fn parens_gives_group_imex() -> Result<()> {
        let i = IMEx::try_from("1(1)(9)*(4){45}(1(1))()")?;

        assert_eq!(
            i,
            IMEx::new(
                vec![
                    QuantifiedIMExVal::new(IMExVal::Single(once(1)), Quantifier::Finite(1),),
                    QuantifiedIMExVal::new(
                        IMExVal::Group(IMEx::new(
                            vec![QuantifiedIMExVal::new(
                                IMExVal::Single(once(1)),
                                Quantifier::Finite(1),
                            )]
                            .into_iter()
                        )),
                        Quantifier::Finite(1),
                    ),
                    QuantifiedIMExVal::new(
                        IMExVal::Group(IMEx::new(
                            vec![QuantifiedIMExVal::new(
                                IMExVal::Single(once(9)),
                                Quantifier::Finite(1),
                            )]
                            .into_iter()
                        )),
                        Quantifier::Infinite,
                    ),
                    QuantifiedIMExVal::new(
                        IMExVal::Group(IMEx::new(
                            vec![QuantifiedIMExVal::new(
                                IMExVal::Single(once(4)),
                                Quantifier::Finite(1),
                            )]
                            .into_iter()
                        )),
                        Quantifier::Finite(45),
                    ),
                    QuantifiedIMExVal::new(
                        IMExVal::Group(IMEx::new(
                            vec![
                                QuantifiedIMExVal::new(
                                    IMExVal::Single(once(1)),
                                    Quantifier::Finite(1),
                                ),
                                QuantifiedIMExVal::new(
                                    IMExVal::Group(IMEx::new(
                                        vec![QuantifiedIMExVal::new(
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
                    QuantifiedIMExVal::new(
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
        IMEx::try_from("0O0").unwrap_err();

        IMEx::try_from("^[0]+$").unwrap_err();

        IMEx::try_from("123*4{5}(x)*").unwrap_err();
    }

    #[test]
    fn too_many_closed_parens_fails() {
        IMEx::try_from("0(1)2)3(4)5").unwrap_err();

        IMEx::try_from("0{1}2}3{4}5").unwrap_err();

        IMEx::try_from("0(1)2(3))").unwrap_err();
    }

    #[test]
    fn too_many_open_parens_fails() {
        IMEx::try_from("0(1)2(3(4)5").unwrap_err();

        IMEx::try_from("0{1}2{3{4}5").unwrap_err();

        IMEx::try_from("0{1}23{4}5{6").unwrap_err();

        IMEx::try_from("((0)1(2)3").unwrap_err();
    }

    #[test]
    fn mismatched_parens_fails() {
        IMEx::try_from(")(").unwrap_err();

        IMEx::try_from("(3{)}").unwrap_err();
    }

    #[test]
    fn bad_repeat_targets_fails() {
        IMEx::try_from("(*4)").unwrap_err();

        IMEx::try_from("({6}6)").unwrap_err();

        IMEx::try_from("*2").unwrap_err();

        IMEx::try_from("{4}4").unwrap_err();

        IMEx::try_from("5{*5}").unwrap_err();
    }

    #[test]
    fn bad_repeat_bracket_contents_fails() {
        IMEx::try_from("5{5*}").unwrap_err();

        IMEx::try_from("6{(6)}").unwrap_err();

        IMEx::try_from("7{7{7}}").unwrap_err();
    }
}
