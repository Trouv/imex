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

fn apply_pattern<T: io::BufRead>(
    pattern: Pattern,
    streams: &mut Vec<io::Lines<T>>,
) -> io::Result<Vec<String>> {
    let mut res: Vec<String> = vec![];

    match pattern.data {
        PatternData::Single(i) => {
            if let Some(s) = streams.get_mut(i) {
                if let Some(l) = s.next() {
                    res.push(l?);
                }
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Pattern item out of file range: {}", i),
                ));
            }
        }
        PatternData::Group(g) => {
            for inner_pattern in g {
                res.extend(apply_pattern(inner_pattern, streams)?);
            }
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
