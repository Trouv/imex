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

    fn from(pattern: &str) -> io::Result<Pattern> {
        Ok(Pattern {
            data: PatternData::Single(1),
            repeater: Repeater::Infinite,
        })
    }
}
