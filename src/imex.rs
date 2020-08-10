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

/// An [`IMExVal`](./enum.IMExVal.html) that has been quantified, for use in a parsed
/// [`IMEx`](./struct.IMEx.html).
#[derive(PartialEq, Debug, Clone)]
pub struct QuantifiedIMExVal {
    pub val: IMExVal,
    pub quantifier: Quantifier,
}

/// A single-element tuple-struct representing a parsed [`IMEx`](./struct.IMEx.html). Used by
/// [`IMExIter`](../merges/trait.IMExMerges.html) to perform lazy merging.
#[derive(PartialEq, Debug, Clone)]
pub struct IMEx(pub Vec<QuantifiedIMExVal>);

impl IMEx {
    /// Parse an [`IMEx`](./struct.IMEx.html) from a string.
    ///
    /// Results in an error if the IMEx is invalid.
    ///
    /// Example:
    /// ```
    /// use ::imex::imex::IMEx;
    /// let imex = IMEx::from("01*(23){4}");
    /// ```
    pub fn from(imex: &str) -> Result<Self> {
        // TODO: Clean this up, implement simple parser combinator?
        let mut sequence: Vec<QuantifiedIMExVal> = vec![];

        let mut in_brackets = false;
        let mut inner_quantifier = String::new();

        let mut parens_depth = 0;
        let mut inner_imex = String::new();
        for c in imex.chars() {
            if parens_depth > 0 {
                if c == '(' {
                    parens_depth += 1;
                } else if c == ')' {
                    parens_depth -= 1;
                }

                if parens_depth == 0 {
                    sequence.push(QuantifiedIMExVal {
                        val: IMExVal::Group(IMEx::from(&inner_imex)?),
                        quantifier: Quantifier::Finite(1),
                    });
                    inner_imex = String::new();
                } else {
                    inner_imex.push(c);
                }
            } else if in_brackets {
                if c.is_digit(10) {
                    inner_quantifier.push(c);
                } else if c == '}' {
                    if let Some(i) = sequence.last_mut() {
                        i.quantifier =
                            Quantifier::Finite(inner_quantifier.parse::<usize>().expect(""));
                        in_brackets = false;
                    } else {
                        return Err(Error::new(InvalidInput, "Bad target for '{/}' quantifier"));
                    }
                } else {
                    return Err(Error::new(
                        InvalidInput,
                        format!(
                            "Only digits can be inside '{{/}}' quantifiers, received: {}",
                            c
                        ),
                    ));
                }
            } else {
                if c.is_digit(10) {
                    sequence.push(QuantifiedIMExVal {
                        val: IMExVal::Single(c.to_digit(10).expect("") as usize),
                        quantifier: Quantifier::Finite(1),
                    });
                } else if c == '(' {
                    parens_depth += 1;
                } else if c == '{' {
                    in_brackets = true;
                } else if c == '*' {
                    if let Some(i) = sequence.last_mut() {
                        i.quantifier = Quantifier::Infinite;
                    } else {
                        return Err(Error::new(InvalidInput, "Bad target for '*' quantifier"));
                    }
                } else {
                    return Err(Error::new(InvalidInput, format!("Bad char in imex: {}", c)));
                }
            }
        }

        if parens_depth > 0 {
            Err(Error::new(
                InvalidInput,
                "Unmatched parentheses, expected ')'",
            ))
        } else if in_brackets {
            Err(Error::new(InvalidInput, "Unmatched brackets, expected '}'"))
        } else {
            Ok(IMEx(sequence))
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
