use crate::repeater::Repeater;
use std::io;

enum PatternData {
    Single(usize),
    Group(Vec<Pattern>),
}

struct Pattern {
    data: PatternData,
    repeater: Repeater,
}

impl Pattern {
    fn from(pattern: &str) -> io::Result<Pattern> {
        Ok(Pattern {
            data: PatternData::Single(1),
            repeater: Repeater::Infinite,
        })
    }

    fn merge_streams<T: io::BufRead>(
        &self,
        streams: &mut Vec<io::Lines<T>>,
    ) -> io::Result<Vec<String>> {
        let mut res: Vec<String> = vec![];

        match &self.data {
            PatternData::Single(i) => {
                self.repeater.repeat(|| -> io::Result<bool> {
                    if let Some(s) = streams.get_mut(*i) {
                        if let Some(l) = s.next() {
                            res.push(l?);
                            Ok(true)
                        } else {
                            Ok(false)
                        }
                    } else {
                        Err(io::Error::new(
                            io::ErrorKind::Other,
                            format!("Pattern item out of file range: {}", i),
                        ))
                    }
                })?;
            }
            PatternData::Group(g) => {
                self.repeater.repeat(|| -> io::Result<bool> {
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
    fn empty_string_gives_empty_group_pattern() {}

    #[test]
    fn repeats_gives_repeating_patterns() {}

    #[test]
    fn parens_gives_group_patterns() {}

    #[test]
    fn bad_chars_fails() {}

    #[test]
    fn too_many_closed_parens_fails() {}

    #[test]
    fn too_many_open_parens_fails() {}

    #[test]
    fn mismatched_parens_fails() {}

    #[test]
    fn bad_repeat_targets_fails() {}

    #[test]
    fn bad_repeat_brace_contents_fails() {}
}

#[cfg(test)]
mod merge_tests {
    use super::*;
}
