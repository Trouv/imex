use crate::quantifier::Quantifier;
use std::io::{Error, ErrorKind::InvalidInput, Result};

#[derive(PartialEq, Debug, Clone)]
pub enum PastexVal {
    Single(usize),
    Group(Pastex),
}

#[derive(PartialEq, Debug, Clone)]
pub struct QuantifiedPastexVal {
    pub val: PastexVal,
    pub quantifier: Quantifier,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Pastex(pub Vec<QuantifiedPastexVal>);

impl Pastex {
    pub fn from(pastex: &str) -> Result<Self> {
        // TODO: Clean this up, implement simple parser combinator?
        let mut sequence: Vec<QuantifiedPastexVal> = vec![];

        let mut in_brackets = false;
        let mut inner_quantifier = String::new();

        let mut parens_depth = 0;
        let mut inner_pastex = String::new();
        for c in pastex.chars() {
            if parens_depth > 0 {
                if c == '(' {
                    parens_depth += 1;
                } else if c == ')' {
                    parens_depth -= 1;
                }

                if parens_depth == 0 {
                    sequence.push(QuantifiedPastexVal {
                        val: PastexVal::Group(Pastex::from(&inner_pastex)?),
                        quantifier: Quantifier::Finite(1),
                    });
                    inner_pastex = String::new();
                } else {
                    inner_pastex.push(c);
                }
            } else if in_brackets {
                if c.is_digit(10) {
                    inner_quantifier.push(c);
                } else if c == '}' {
                    if let Some(p) = sequence.last_mut() {
                        p.quantifier =
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
                    sequence.push(QuantifiedPastexVal {
                        val: PastexVal::Single(c.to_digit(10).expect("") as usize),
                        quantifier: Quantifier::Finite(1),
                    });
                } else if c == '(' {
                    parens_depth += 1;
                } else if c == '{' {
                    in_brackets = true;
                } else if c == '*' {
                    if let Some(p) = sequence.last_mut() {
                        p.quantifier = Quantifier::Infinite;
                    } else {
                        return Err(Error::new(InvalidInput, "Bad target for '*' quantifier"));
                    }
                } else {
                    return Err(Error::new(
                        InvalidInput,
                        format!("Bad char in pastex: {}", c),
                    ));
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
            Ok(Pastex(sequence))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string_gives_empty_group_pastex() -> Result<()> {
        let p = Pastex::from("")?;

        assert_eq!(p, Pastex(vec![]));
        Ok(())
    }

    #[test]
    fn repeats_gives_repeating_pastex() -> Result<()> {
        let p = Pastex::from("13{3}9*1")?;

        assert_eq!(
            p,
            Pastex(vec![
                QuantifiedPastexVal {
                    val: PastexVal::Single(1),
                    quantifier: Quantifier::Finite(1),
                },
                QuantifiedPastexVal {
                    val: PastexVal::Single(3),
                    quantifier: Quantifier::Finite(3),
                },
                QuantifiedPastexVal {
                    val: PastexVal::Single(9),
                    quantifier: Quantifier::Infinite,
                },
                QuantifiedPastexVal {
                    val: PastexVal::Single(1),
                    quantifier: Quantifier::Finite(1),
                },
            ],)
        );
        Ok(())
    }

    #[test]
    fn parens_gives_group_pastex() -> Result<()> {
        let p = Pastex::from("1(1)(9)*(4){4}(1(1))()")?;

        assert_eq!(
            p,
            Pastex(vec![
                QuantifiedPastexVal {
                    val: PastexVal::Single(1),
                    quantifier: Quantifier::Finite(1),
                },
                QuantifiedPastexVal {
                    val: PastexVal::Group(Pastex(vec![QuantifiedPastexVal {
                        val: PastexVal::Single(1),
                        quantifier: Quantifier::Finite(1),
                    }])),
                    quantifier: Quantifier::Finite(1),
                },
                QuantifiedPastexVal {
                    val: PastexVal::Group(Pastex(vec![QuantifiedPastexVal {
                        val: PastexVal::Single(9),
                        quantifier: Quantifier::Finite(1),
                    }])),
                    quantifier: Quantifier::Infinite,
                },
                QuantifiedPastexVal {
                    val: PastexVal::Group(Pastex(vec![QuantifiedPastexVal {
                        val: PastexVal::Single(4),
                        quantifier: Quantifier::Finite(1),
                    }])),
                    quantifier: Quantifier::Finite(4),
                },
                QuantifiedPastexVal {
                    val: PastexVal::Group(Pastex(vec![
                        QuantifiedPastexVal {
                            val: PastexVal::Single(1),
                            quantifier: Quantifier::Finite(1),
                        },
                        QuantifiedPastexVal {
                            val: PastexVal::Group(Pastex(vec![QuantifiedPastexVal {
                                val: PastexVal::Single(1),
                                quantifier: Quantifier::Finite(1),
                            }])),
                            quantifier: Quantifier::Finite(1),
                        }
                    ])),
                    quantifier: Quantifier::Finite(1),
                },
                QuantifiedPastexVal {
                    val: PastexVal::Group(Pastex(vec![])),
                    quantifier: Quantifier::Finite(1),
                }
            ])
        );
        Ok(())
    }

    #[test]
    fn bad_chars_fails() {
        Pastex::from("0O0").unwrap_err();

        Pastex::from("^[0]+$").unwrap_err();

        Pastex::from("123*4{5}(x)*").unwrap_err();
    }

    #[test]
    fn too_many_closed_parens_fails() {
        Pastex::from("0(1)2)3(4)5").unwrap_err();

        Pastex::from("0{1}2}3{4}5").unwrap_err();

        Pastex::from("0(1)2(3))").unwrap_err();
    }

    #[test]
    fn too_many_open_parens_fails() {
        Pastex::from("0(1)2(3(4)5").unwrap_err();

        Pastex::from("0{1}2{3{4}5").unwrap_err();

        Pastex::from("0{1}23{4}5{6").unwrap_err();

        Pastex::from("((0)1(2)3").unwrap_err();
    }

    #[test]
    fn mismatched_parens_fails() {
        Pastex::from(")(").unwrap_err();

        Pastex::from("(3{)}").unwrap_err();
    }

    #[test]
    fn bad_repeat_targets_fails() {
        Pastex::from("(*4)").unwrap_err();

        Pastex::from("({6}6)").unwrap_err();

        Pastex::from("*2").unwrap_err();

        Pastex::from("{4}4").unwrap_err();

        Pastex::from("5{*5}").unwrap_err();
    }

    #[test]
    fn bad_repeat_bracket_contents_fails() {
        Pastex::from("5{5*}").unwrap_err();

        Pastex::from("6{(6)}").unwrap_err();

        Pastex::from("7{7{7}}").unwrap_err();
    }
}
