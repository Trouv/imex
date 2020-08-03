use crate::repeater::Repeater;
use std::borrow::Borrow;
use std::io::{Error, ErrorKind::InvalidInput, Result};

#[derive(PartialEq, Debug)]
enum ZprexVal {
    Single(usize),
    Group(Vec<Zprex>),
}

#[derive(PartialEq, Debug)]
pub struct Zprex {
    val: ZprexVal,
    repeater: Repeater,
}

impl Zprex {
    pub fn from(zprex: &str) -> Result<Self> {
        // TODO: Clean this up, implement simple parser combinator?
        let mut group = vec![];

        let mut in_brackets = false;
        let mut inner_quantifier = String::new();

        let mut parens_depth = 0;
        let mut inner_zprex = String::new();
        for c in zprex.chars() {
            if parens_depth > 0 {
                if c == '(' {
                    parens_depth += 1;
                } else if c == ')' {
                    parens_depth -= 1;
                }

                if parens_depth == 0 {
                    group.push(Zprex::from(&inner_zprex)?);
                    inner_zprex = String::new();
                } else {
                    inner_zprex.push(c);
                }
            } else if in_brackets {
                if c.is_digit(10) {
                    inner_quantifier.push(c);
                } else if c == '}' {
                    if let Some(p) = group.last_mut() {
                        p.repeater = Repeater::Finite(inner_quantifier.parse::<usize>().expect(""));
                        in_brackets = false;
                    } else {
                        return Err(Error::new(InvalidInput, "Bad target for '{/}' repeater"));
                    }
                } else {
                    return Err(Error::new(
                        InvalidInput,
                        format!(
                            "Only digits can be inside '{{/}}' repeaters, received: {}",
                            c
                        ),
                    ));
                }
            } else {
                if c.is_digit(10) {
                    group.push(Zprex {
                        val: ZprexVal::Single(c.to_digit(10).expect("") as usize),
                        repeater: Repeater::Finite(1),
                    });
                } else if c == '(' {
                    parens_depth += 1;
                } else if c == '{' {
                    in_brackets = true;
                } else if c == '*' {
                    if let Some(p) = group.last_mut() {
                        p.repeater = Repeater::Infinite;
                    } else {
                        return Err(Error::new(InvalidInput, "Bad target for '*' repeater"));
                    }
                } else {
                    return Err(Error::new(
                        InvalidInput,
                        format!("Bad char in zprex: {}", c),
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
            Ok(Zprex {
                val: ZprexVal::Group(group),
                repeater: Repeater::Finite(1),
            })
        }
    }

    pub fn merge_iters<I>(&self, iters: &mut Vec<I>) -> Result<Vec<String>>
    // TODO: Reconsider this return type
    // TODO: lazy evaluation, impl Iterator
    where
        I: Iterator,
        I::Item: Borrow<str>,
    {
        let mut res: Vec<String> = vec![];

        match &self.val {
            ZprexVal::Single(i) => {
                self.repeater.repeat(|| -> Result<bool> {
                    if let Some(s) = iters.get_mut(*i) {
                        if let Some(l) = s.next() {
                            res.push(l.borrow().to_string());
                            Ok(true)
                        } else {
                            Ok(false)
                        }
                    } else {
                        Err(Error::new(
                            InvalidInput,
                            format!("Zprex item out of file range: {}", i),
                        ))
                    }
                })?;
            }
            ZprexVal::Group(g) => {
                self.repeater.repeat(|| -> Result<bool> {
                    let mut rep = false;
                    for inner_zprex in g {
                        let mut inner_res = inner_zprex.merge_iters(iters)?;
                        if inner_res.len() > 0 {
                            rep = true;
                        }
                        res.append(&mut inner_res);
                    }
                    Ok(rep)
                })?;
            }
        }

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string_gives_empty_group_zprex() -> Result<()> {
        let p = Zprex::from("")?;

        assert_eq!(
            p,
            Zprex {
                val: ZprexVal::Group(vec![]),
                repeater: Repeater::Finite(1),
            }
        );
        Ok(())
    }

    #[test]
    fn repeats_gives_repeating_zprex() -> Result<()> {
        let p = Zprex::from("13{3}9*1")?;

        assert_eq!(
            p,
            Zprex {
                val: ZprexVal::Group(vec![
                    Zprex {
                        val: ZprexVal::Single(1),
                        repeater: Repeater::Finite(1),
                    },
                    Zprex {
                        val: ZprexVal::Single(3),
                        repeater: Repeater::Finite(3),
                    },
                    Zprex {
                        val: ZprexVal::Single(9),
                        repeater: Repeater::Infinite,
                    },
                    Zprex {
                        val: ZprexVal::Single(1),
                        repeater: Repeater::Finite(1),
                    },
                ]),
                repeater: Repeater::Finite(1),
            }
        );
        Ok(())
    }

    #[test]
    fn parens_gives_group_zprex() -> Result<()> {
        let p = Zprex::from("1(1)(9)*(4){4}(1(1))()")?;

        assert_eq!(
            p,
            Zprex {
                val: ZprexVal::Group(vec![
                    Zprex {
                        val: ZprexVal::Single(1),
                        repeater: Repeater::Finite(1),
                    },
                    Zprex {
                        val: ZprexVal::Group(vec![Zprex {
                            val: ZprexVal::Single(1),
                            repeater: Repeater::Finite(1),
                        }]),
                        repeater: Repeater::Finite(1),
                    },
                    Zprex {
                        val: ZprexVal::Group(vec![Zprex {
                            val: ZprexVal::Single(9),
                            repeater: Repeater::Finite(1),
                        }]),
                        repeater: Repeater::Infinite,
                    },
                    Zprex {
                        val: ZprexVal::Group(vec![Zprex {
                            val: ZprexVal::Single(4),
                            repeater: Repeater::Finite(1),
                        }]),
                        repeater: Repeater::Finite(4),
                    },
                    Zprex {
                        val: ZprexVal::Group(vec![
                            Zprex {
                                val: ZprexVal::Single(1),
                                repeater: Repeater::Finite(1),
                            },
                            Zprex {
                                val: ZprexVal::Group(vec![Zprex {
                                    val: ZprexVal::Single(1),
                                    repeater: Repeater::Finite(1),
                                }]),
                                repeater: Repeater::Finite(1),
                            }
                        ]),
                        repeater: Repeater::Finite(1),
                    },
                    Zprex {
                        val: ZprexVal::Group(vec![]),
                        repeater: Repeater::Finite(1),
                    }
                ]),
                repeater: Repeater::Finite(1),
            }
        );
        Ok(())
    }

    #[test]
    fn bad_chars_fails() {
        Zprex::from("0O0").unwrap_err();

        Zprex::from("^[0]+$").unwrap_err();

        Zprex::from("123*4{5}(x)*").unwrap_err();
    }

    #[test]
    fn too_many_closed_parens_fails() {
        Zprex::from("0(1)2)3(4)5").unwrap_err();

        Zprex::from("0{1}2}3{4}5").unwrap_err();

        Zprex::from("0(1)2(3))").unwrap_err();
    }

    #[test]
    fn too_many_open_parens_fails() {
        Zprex::from("0(1)2(3(4)5").unwrap_err();

        Zprex::from("0{1}2{3{4}5").unwrap_err();

        Zprex::from("0{1}23{4}5{6").unwrap_err();

        Zprex::from("((0)1(2)3").unwrap_err();
    }

    #[test]
    fn mismatched_parens_fails() {
        Zprex::from(")(").unwrap_err();

        Zprex::from("(3{)}").unwrap_err();
    }

    #[test]
    fn bad_repeat_targets_fails() {
        Zprex::from("(*4)").unwrap_err();

        Zprex::from("({6}6)").unwrap_err();

        Zprex::from("*2").unwrap_err();

        Zprex::from("{4}4").unwrap_err();

        Zprex::from("5{*5}").unwrap_err();
    }

    #[test]
    fn bad_repeat_bracket_contents_fails() {
        Zprex::from("5{5*}").unwrap_err();

        Zprex::from("6{(6)}").unwrap_err();

        Zprex::from("7{7{7}}").unwrap_err();
    }
}