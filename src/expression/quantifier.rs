use super::ParserCombinator;
use nom::{
    branch::alt,
    character::complete::{char, digit1},
    combinator::opt,
    sequence::delimited,
    IResult,
};

/// Represents a quantifier in a parsed [`IMEx`](../imex/struct.IMEx.html). Either Finite (`{x}`), in
/// which case a range is contained, or Infinite (`*`).
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Quantifier {
    Infinite,
    Finite(usize),
}

impl Iterator for Quantifier {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        match *self {
            Quantifier::Finite(0) => None,
            Quantifier::Finite(n) => {
                *self = Quantifier::Finite(n - 1);
                Some(())
            }
            _ => Some(()),
        }
    }
}

fn parse_finite_quantifier(input: &str) -> IResult<&str, Quantifier> {
    match opt(delimited(char('{'), digit1, char('}')))(input)? {
        (input, Some(x)) => Ok((
            input,
            Quantifier::Finite(x.parse::<usize>().expect("Expected value to be a digit")),
        )),
        (input, None) => Ok((input, Quantifier::Finite(1))),
    }
}

fn parse_infinite_quantifier(input: &str) -> IResult<&str, Quantifier> {
    let (input, _) = char('*')(input)?;
    Ok((input, Quantifier::Infinite))
}

impl ParserCombinator for Quantifier {
    fn parse(input: &str) -> IResult<&str, Quantifier> {
        alt((parse_infinite_quantifier, parse_finite_quantifier))(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_quantifier_iterates_thrice() {
        let mut r = Quantifier::Finite(3);
        assert_eq!(r.next(), Some(()));
        assert_eq!(r, Quantifier::Finite(2));
        assert_eq!(r.next(), Some(()));
        assert_eq!(r, Quantifier::Finite(1));
        assert_eq!(r.next(), Some(()));
        assert_eq!(r, Quantifier::Finite(0));
        assert_eq!(r.next(), None);
    }

    #[test]
    fn zero_quantifier_gives_immediate_none() {
        let mut r = Quantifier::Finite(0);
        assert_eq!(r.next(), None);
    }

    #[test]
    fn infinite_quantifier_iterates_a_lot() {
        let mut r = Quantifier::Infinite;
        for _ in 0..100 {
            assert_eq!(r.next(), Some(()));
        }
    }
}
