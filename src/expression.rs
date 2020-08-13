use crate::quantifier::Quantifier;
use std::io::{Error, ErrorKind::InvalidInput, Result};

/// [`IMEx`]: ./struct.IMEx.html
/// Represents a quantifiable value in a parsed [`IMEx`]. So, this is either a Single, which
/// contains a digit for indexing iterators, or a Group, which contains an inner parsed [`IMEx`].
#[derive(PartialEq, Debug, Clone)]
pub enum IMExVal {
    Single(usize),
    Group(IMEx),
}

impl IMExVal {
    fn parse(input: String) -> Result<(IMExVal, String)> {
        let mut chars = input.chars();
        match chars.next() {
            Some('(') => {
                let (imex, input) = IMEx::parse_in_parens(chars.collect())?;
                Ok((IMExVal::Group(imex), input))
            }
            Some(x) if x.is_digit(10) => Ok((
                IMExVal::Single(x.to_digit(10).expect("Expected value to be a digit") as usize),
                chars.collect(),
            )),
            _ => Err(Error::new(
                InvalidInput,
                "Expected either a digit or a group",
            )),
        }
    }
}

/// An [`IMExVal`](./enum.IMExVal.html) that has been quantified, for use in a parsed
/// [`IMEx`](./struct.IMEx.html).
#[derive(PartialEq, Debug, Clone)]
pub struct QuantifiedIMExVal {
    pub val: IMExVal,
    pub quantifier: Quantifier,
}

impl QuantifiedIMExVal {
    fn parse(input: String) -> Result<(QuantifiedIMExVal, String)> {
        let (val, input) = IMExVal::parse(input)?;
        let (quantifier, input) = Quantifier::parse(input)?;
        Ok((QuantifiedIMExVal { val, quantifier }, input))
    }
}

/// A single-element tuple-struct representing a parsed [`IMEx`](./struct.IMEx.html). Used by
/// [`IMExIter`](../merges/trait.IMExMerges.html) to perform lazy merging.
#[derive(PartialEq, Debug, Clone)]
pub struct IMEx(pub Vec<QuantifiedIMExVal>);

impl IMEx {
    /// Parse an [`IMEx`](./struct.IMEx.html) from a string.
    ///
    /// # Error
    /// Results in an error if the IMEx is invalid.
    ///
    /// # Example
    /// ```
    /// use imex::IMEx;
    /// let imex = IMEx::from("01*(23){4}");
    /// ```
    pub fn from(imex: &str) -> Result<Self> {
        Ok(IMEx::parse(imex.to_string())?.0)
    }

    fn parse_in_parens(input: String) -> Result<(IMEx, String)> {
        let mut imex = Vec::<QuantifiedIMExVal>::new();
        let mut input = input;
        loop {
            println!("{:?}", imex);
            let mut chars = input.chars();
            match chars.next() {
                Some(')') => return Ok((IMEx(imex), chars.collect())),
                _ => {
                    let (qimexval, s) = QuantifiedIMExVal::parse(input)?;
                    input = s;
                    imex.push(qimexval);
                }
            }
        }
    }

    fn parse(input: String) -> Result<(IMEx, String)> {
        let mut imex = Vec::<QuantifiedIMExVal>::new();
        let mut input = input;
        loop {
            println!("{:?}", imex);
            let mut chars = input.chars();
            match chars.next() {
                None => return Ok((IMEx(imex), chars.collect())),
                _ => {
                    let (qimexval, s) = QuantifiedIMExVal::parse(input)?;
                    input = s;
                    imex.push(qimexval);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string_gives_empty_group_imex() -> Result<()> {
        let i = IMEx::from("")?;

        assert_eq!(i, IMEx(vec![]));
        Ok(())
    }

    #[test]
    fn repeats_gives_repeating_imex() -> Result<()> {
        let i = IMEx::from("13{3}9*1")?;

        assert_eq!(
            i,
            IMEx(vec![
                QuantifiedIMExVal {
                    val: IMExVal::Single(1),
                    quantifier: Quantifier::Finite(1),
                },
                QuantifiedIMExVal {
                    val: IMExVal::Single(3),
                    quantifier: Quantifier::Finite(3),
                },
                QuantifiedIMExVal {
                    val: IMExVal::Single(9),
                    quantifier: Quantifier::Infinite,
                },
                QuantifiedIMExVal {
                    val: IMExVal::Single(1),
                    quantifier: Quantifier::Finite(1),
                },
            ],)
        );
        Ok(())
    }

    #[test]
    fn parens_gives_group_imex() -> Result<()> {
        let i = IMEx::from("1(1)(9)*(4){4}(1(1))()")?;

        assert_eq!(
            i,
            IMEx(vec![
                QuantifiedIMExVal {
                    val: IMExVal::Single(1),
                    quantifier: Quantifier::Finite(1),
                },
                QuantifiedIMExVal {
                    val: IMExVal::Group(IMEx(vec![QuantifiedIMExVal {
                        val: IMExVal::Single(1),
                        quantifier: Quantifier::Finite(1),
                    }])),
                    quantifier: Quantifier::Finite(1),
                },
                QuantifiedIMExVal {
                    val: IMExVal::Group(IMEx(vec![QuantifiedIMExVal {
                        val: IMExVal::Single(9),
                        quantifier: Quantifier::Finite(1),
                    }])),
                    quantifier: Quantifier::Infinite,
                },
                QuantifiedIMExVal {
                    val: IMExVal::Group(IMEx(vec![QuantifiedIMExVal {
                        val: IMExVal::Single(4),
                        quantifier: Quantifier::Finite(1),
                    }])),
                    quantifier: Quantifier::Finite(4),
                },
                QuantifiedIMExVal {
                    val: IMExVal::Group(IMEx(vec![
                        QuantifiedIMExVal {
                            val: IMExVal::Single(1),
                            quantifier: Quantifier::Finite(1),
                        },
                        QuantifiedIMExVal {
                            val: IMExVal::Group(IMEx(vec![QuantifiedIMExVal {
                                val: IMExVal::Single(1),
                                quantifier: Quantifier::Finite(1),
                            }])),
                            quantifier: Quantifier::Finite(1),
                        }
                    ])),
                    quantifier: Quantifier::Finite(1),
                },
                QuantifiedIMExVal {
                    val: IMExVal::Group(IMEx(vec![])),
                    quantifier: Quantifier::Finite(1),
                }
            ])
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
