use crate::repeater::Repeater;
use std::io::{BufRead, Error, ErrorKind::InvalidInput, Lines, Result};

#[derive(PartialEq, Debug)]
enum PatternData {
    Single(usize),
    Group(Vec<Pattern>),
}

#[derive(PartialEq, Debug)]
struct Pattern {
    data: PatternData,
    repeater: Repeater,
}

impl Pattern {
    fn from(pattern: &str) -> Result<Pattern> {
        Ok(Pattern {
            data: PatternData::Single(1),
            repeater: Repeater::Infinite,
        })
    }

    fn merge_streams<T: BufRead>(&self, streams: &mut Vec<Lines<T>>) -> Result<Vec<String>> {
        let mut res: Vec<String> = vec![];

        match &self.data {
            PatternData::Single(i) => {
                self.repeater.repeat(|| -> Result<bool> {
                    if let Some(s) = streams.get_mut(*i) {
                        if let Some(l) = s.next() {
                            res.push(l?);
                            Ok(true)
                        } else {
                            Ok(false)
                        }
                    } else {
                        Err(Error::new(
                            InvalidInput,
                            format!("Pattern item out of file range: {}", i),
                        ))
                    }
                })?;
            }
            PatternData::Group(g) => {
                self.repeater.repeat(|| -> Result<bool> {
                    let mut rep = false;
                    for inner_pattern in g {
                        let inner_res = inner_pattern.merge_streams(streams)?;
                        if inner_res.len() > 0 {
                            rep = true;
                        }
                    }
                    Ok(rep)
                })?;
            }
        }

        Ok(res)
    }
}

#[cfg(test)]
mod parse_tests {
    use super::*;

    #[test]
    fn empty_string_gives_empty_group_pattern() -> Result<()> {
        let p = Pattern::from("")?;

        assert_eq!(
            p,
            Pattern {
                data: PatternData::Group(vec![]),
                repeater: Repeater::Finite(1),
            }
        );
        Ok(())
    }

    #[test]
    fn repeats_gives_repeating_patterns() -> Result<()> {
        let p = Pattern::from("13{3}9*1")?;

        assert_eq!(
            p,
            Pattern {
                data: PatternData::Group(vec![
                    Pattern {
                        data: PatternData::Single(1),
                        repeater: Repeater::Finite(1),
                    },
                    Pattern {
                        data: PatternData::Single(3),
                        repeater: Repeater::Finite(3),
                    },
                    Pattern {
                        data: PatternData::Single(9),
                        repeater: Repeater::Infinite,
                    },
                    Pattern {
                        data: PatternData::Single(1),
                        repeater: Repeater::Finite(1),
                    },
                ]),
                repeater: Repeater::Finite(1),
            }
        );
        Ok(())
    }

    #[test]
    fn parens_gives_group_patterns() -> Result<()> {
        let p = Pattern::from("1(1)(9)*(4){4}(1(1))()")?;

        assert_eq!(
            p,
            Pattern {
                data: PatternData::Group(vec![
                    Pattern {
                        data: PatternData::Single(1),
                        repeater: Repeater::Finite(1),
                    },
                    Pattern {
                        data: PatternData::Group(vec![Pattern {
                            data: PatternData::Single(1),
                            repeater: Repeater::Finite(1),
                        }]),
                        repeater: Repeater::Finite(1),
                    },
                    Pattern {
                        data: PatternData::Group(vec![Pattern {
                            data: PatternData::Single(9),
                            repeater: Repeater::Finite(1),
                        }]),
                        repeater: Repeater::Infinite,
                    },
                    Pattern {
                        data: PatternData::Group(vec![Pattern {
                            data: PatternData::Single(4),
                            repeater: Repeater::Finite(1),
                        }]),
                        repeater: Repeater::Finite(4),
                    },
                    Pattern {
                        data: PatternData::Group(vec![
                            Pattern {
                                data: PatternData::Single(1),
                                repeater: Repeater::Finite(1),
                            },
                            Pattern {
                                data: PatternData::Group(vec![Pattern {
                                    data: PatternData::Single(1),
                                    repeater: Repeater::Finite(1),
                                }]),
                                repeater: Repeater::Finite(1),
                            }
                        ]),
                        repeater: Repeater::Finite(1),
                    },
                    Pattern {
                        data: PatternData::Group(vec![]),
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
        Pattern::from("0O0").unwrap_err();

        Pattern::from("^[0]+$").unwrap_err();

        Pattern::from("123*4{5}(x)*").unwrap_err();
    }

    #[test]
    fn too_many_closed_parens_fails() {
        Pattern::from("0(1)2)3(4)5").unwrap_err();

        Pattern::from("0{1}2}3{4}5").unwrap_err();

        Pattern::from("0(1)2(3))").unwrap_err();
    }

    #[test]
    fn too_many_open_parens_fails() {
        Pattern::from("0(1)2(3(4)5").unwrap_err();

        Pattern::from("0{1}2{3{4}5").unwrap_err();

        Pattern::from("((0)1(2)3").unwrap_err();
    }

    #[test]
    fn mismatched_parens_fails() {
        Pattern::from(")(").unwrap_err();

        Pattern::from("(3{)}").unwrap_err();
    }

    #[test]
    fn bad_repeat_targets_fails() {
        Pattern::from("(*4)").unwrap_err();

        Pattern::from("({6}6)").unwrap_err();

        Pattern::from("*2").unwrap_err();

        Pattern::from("{4}4").unwrap_err();

        Pattern::from("5{5}*").unwrap_err();

        Pattern::from("5*{5}").unwrap_err();

        Pattern::from("5{*5}").unwrap_err();
    }

    #[test]
    fn bad_repeat_brace_contents_fails() {
        Pattern::from("5{5*}").unwrap_err();

        Pattern::from("6{(6)}").unwrap_err();

        Pattern::from("7{7{7}}").unwrap_err();
    }
}

#[cfg(test)]
mod merge_tests {
    use super::*;
}
