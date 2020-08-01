use crate::repeater::Repeater;
use std::borrow::Borrow;
use std::io::{Error, ErrorKind::InvalidInput, Result};

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
        // TODO: Clean this up, implement simple parser combinator?
        let mut group = vec![];

        let mut in_brackets = false;
        let mut inner_quantifier = String::new();

        let mut parens_depth = 0;
        let mut inner_pattern = String::new();
        for c in pattern.chars() {
            if parens_depth > 0 {
                if c == '(' {
                    parens_depth += 1;
                } else if c == ')' {
                    parens_depth -= 1;
                }

                if parens_depth == 0 {
                    group.push(Pattern::from(&inner_pattern)?);
                    inner_pattern = String::new();
                } else {
                    inner_pattern.push(c);
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
                    group.push(Pattern {
                        data: PatternData::Single(c.to_digit(10).expect("") as usize),
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
                        format!("Bad char in pattern: {}", c),
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
            Ok(Pattern {
                data: PatternData::Group(group),
                repeater: Repeater::Finite(1),
            })
        }
    }

    fn merge_streams<I>(&self, streams: &mut Vec<I>) -> Result<Vec<String>>
    // TODO: Reconsider this return type
    // TODO: lazy evaluation, impl Iterator
    where
        I: Iterator,
        I::Item: Borrow<str>,
    {
        let mut res: Vec<String> = vec![];

        match &self.data {
            PatternData::Single(i) => {
                self.repeater.repeat(|| -> Result<bool> {
                    if let Some(s) = streams.get_mut(*i) {
                        if let Some(l) = s.next() {
                            res.push(l.borrow().to_string());
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
                        let mut inner_res = inner_pattern.merge_streams(streams)?;
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

        Pattern::from("0{1}23{4}5{6").unwrap_err();

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

        Pattern::from("5{*5}").unwrap_err();
    }

    #[test]
    fn bad_repeat_bracket_contents_fails() {
        Pattern::from("5{5*}").unwrap_err();

        Pattern::from("6{(6)}").unwrap_err();

        Pattern::from("7{7{7}}").unwrap_err();
    }
}

#[cfg(test)]
mod merge_tests {
    use super::*;

    #[test]
    fn non_repeating_pattern_might_not_complete() -> Result<()> {
        let p = Pattern::from("01(10){3}")?;
        let mut streams = vec!["0\n0\n0\n0\n0".lines(), "1\n1\n1\n1\n1".lines()];

        assert_eq!(
            p.merge_streams(&mut streams)?,
            vec!["0", "1", "1", "0", "1", "0", "1", "0"]
        );

        Ok(())
    }

    #[test]
    fn repeating_patterns_repeat() -> Result<()> {
        let p = Pattern::from("0{3}1(01){5}")?;
        let mut streams = vec!["0\n0\n0\n0\n0\n0\n0\n0".lines(), "1\n1\n1\n1\n1\n1".lines()];

        assert_eq!(
            p.merge_streams(&mut streams)?,
            vec!["0", "0", "0", "1", "0", "1", "0", "1", "0", "1", "0", "1", "0", "1"]
        );

        Ok(())
    }

    #[test]
    fn completed_pattern_exits_repeating() -> Result<()> {
        let p = Pattern::from("0*(12)*")?;
        let mut streams = vec![
            "0\n0\n0".lines(),
            "1\n1\n1".lines(),
            "2\n2\n2\n2\n2".lines(),
        ];

        assert_eq!(
            p.merge_streams(&mut streams)?,
            vec!["0", "0", "0", "1", "2", "1", "2", "1", "2", "2", "2"]
        );

        Ok(())
    }

    #[test]
    fn out_of_range_pattern_fails() -> Result<()> {
        let p = Pattern::from("0120")?;
        let mut streams = vec!["0\n0\n0".lines(), "1\n1\n1".lines()];

        p.merge_streams(&mut streams).unwrap_err();

        Ok(())
    }

    #[test]
    fn empty_pattern_gives_empty_merge() -> Result<()> {
        let p = Pattern::from("")?;
        let mut streams = vec!["0\n0\n0".lines(), "1\n1\n1".lines()];

        assert_eq!(p.merge_streams(&mut streams)?, Vec::<String>::new());

        Ok(())
    }

    #[test]
    fn empty_streams_give_empty_merge() -> Result<()> {
        let p = Pattern::from("0120")?;
        let mut streams = vec!["".lines(), "".lines(), "".lines()];

        assert_eq!(p.merge_streams(&mut streams)?, Vec::<String>::new());

        Ok(())
    }

    #[test]
    fn empty_stream_list_only_passes_for_empty_pattern() -> Result<()> {
        let p = Pattern::from("0120")?;
        let mut streams: Vec<std::str::Lines> = vec![];

        p.merge_streams(&mut streams).unwrap_err();

        let p = Pattern::from("")?;

        assert_eq!(p.merge_streams(&mut streams)?, Vec::<String>::new());

        Ok(())
    }

    // TODO: Tests for str::Lines vs io::Lines?
}
