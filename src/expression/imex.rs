use super::{IMExIterator, ParserCombinator, QuantifiedIMExVal};
use nom::{
    character::complete::char,
    combinator::all_consuming,
    multi::{many0, many_till},
    IResult,
};
use std::{
    convert::TryFrom,
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
    pub fn new(vals: IntoIter<QuantifiedIMExVal>) -> IMEx {
        IMEx {
            vals,
            current_val: None,
        }
    }

    /// Parser combinator for parsing an [`IMEx`](../imex/struct.IMEx.html), making use of the
    /// [`nom`](https://docs.rs/nom/6.0.0-alpha1/nom/index.html) library. Unless you're building your
    /// own parser that incorporates IMExes using parser combinators, you may prefer to use
    /// [`IMEx::try_from`](../imex/struct.IMEx.html#method.from), which uses this function but loses the
    /// parser combinator details.
    ///
    /// # Error
    /// Results in an error if the input string is not a valid IMEx.
    ///
    /// Currently, this parser combinator expects to be "all consuming", which means it will fail if
    /// there is any input string remaining after parsing an IMEx. This could pose compatibility issues
    /// if you want to use this in your own set of parser combinators. If this is a use case for you,
    /// consider contributing to this project on [github](https://github.com/Trouv/imex).
    pub fn parse_complete(input: &str) -> IResult<&str, IMEx> {
        let (input, imex) = all_consuming(many0(QuantifiedIMExVal::parse))(input)?;
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
            Err(e) => Err(Error::new(InvalidInput, format!("{}", e))),
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
    fn parse(input: &str) -> IResult<&str, IMEx> {
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
        let i = IMEx::try_from("1(1)(9)*(4){45}(1(1))()")?;

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
