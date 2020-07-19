use std::io;

enum Repeat {
    Infinitely,
    Finitely(usize),
}

enum PatternData {
    Single(usize),
    Group(Vec<Pattern>),
}

struct Pattern {
    data: PatternData,
    repeat: Repeat,
}

impl Repeat {
    fn apply<F>(&self, mut op: F) -> io::Result<()>
    where
        F: FnMut() -> io::Result<bool>,
    {
        let mut repeat = true;
        let mut rep_count: usize = 0;
        while repeat {
            repeat = op()?;
            rep_count += 1;

            if let Repeat::Finitely(x) = self {
                if rep_count >= *x {
                    repeat = false;
                }
            }
        }
        Ok(())
    }
}

fn apply_pattern<T: io::BufRead>(
    pattern: &Pattern,
    streams: &mut Vec<io::Lines<T>>,
) -> io::Result<Vec<String>> {
    let mut res: Vec<String> = vec![];

    match &pattern.data {
        PatternData::Single(i) => {
            pattern.repeat.apply(|| -> io::Result<bool> {
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
            pattern.repeat.apply(|| -> io::Result<bool> {
                let mut rep = false;
                for inner_pattern in g {
                    let inner_res = apply_pattern(&inner_pattern, streams)?;
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

fn parse_pattern(pattern: &str) -> io::Result<Pattern> {
    Ok(Pattern {
        data: PatternData::Single(1),
        repeat: Repeat::Infinitely,
    })
}

#[cfg(test)]
mod tests {}
