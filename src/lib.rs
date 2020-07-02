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

impl Iterator for Pattern {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        Some(1)
    }
}

fn apply_pattern<T: Iterator>(pattern: Pattern, streams: Vec<T>) -> io::Result<Vec<String>> {
    Ok(vec![])
}

fn parse_pattern(pattern: &str) -> io::Result<Pattern> {
    Ok(Pattern {
        data: PatternData::Single(1),
        repeat: Repeat::Infinitely,
    })
}
