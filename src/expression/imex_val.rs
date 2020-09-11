use super::{IMEx, IMExIterator, ParserCombinator};
use nom::{
    branch::alt,
    character::complete::{char, one_of},
    IResult,
};
use std::iter::{once, Once};

/// [`IMEx`]: ./struct.IMEx.html
/// Represents a quantifiable value in a parsed [`IMEx`]. So, this is either a Single, which
/// contains a digit for indexing iterators, or a Group, which contains an inner parsed [`IMEx`].
#[derive(Debug, Clone)]
pub enum IMExVal {
    Single(Once<usize>),
    Group(IMEx),
}

impl PartialEq for IMExVal {
    fn eq(&self, other: &IMExVal) -> bool {
        match (self, other) {
            (IMExVal::Single(a), IMExVal::Single(b)) => a.clone().next() == b.clone().next(),
            (IMExVal::Group(a), IMExVal::Group(b)) => a.eq(b),
            _ => false,
        }
    }
}

impl IMExIterator for IMExVal {
    fn iterate<T, I>(&mut self, iters: &mut Vec<T>) -> Option<I>
    where
        T: Iterator<Item = I>,
    {
        match self {
            IMExVal::Single(once) => match once.next() {
                Some(index) => match iters.get_mut(index) {
                    Some(iter) => iter.next(),
                    None => None,
                },
                None => None,
            },
            IMExVal::Group(imex) => imex.iterate(iters),
        }
    }
}

fn parse_single_imex_val(input: &str) -> IResult<&str, IMExVal> {
    let (input, x) = one_of("0123456789")(input)?;
    Ok((
        input,
        IMExVal::Single(once(
            x.to_digit(10).expect("Expected value to be a digit") as usize
        )),
    ))
}

fn parse_group_imex_val(input: &str) -> IResult<&str, IMExVal> {
    let (input, _) = char('(')(input)?;
    let (input, imex) = IMEx::parse(input)?;
    Ok((input, IMExVal::Group(imex)))
}

impl ParserCombinator for IMExVal {
    fn parse(input: &str) -> IResult<&str, IMExVal> {
        alt((parse_single_imex_val, parse_group_imex_val))(input)
    }
}
