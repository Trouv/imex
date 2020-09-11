pub trait IMExIterator {
    type Item;
    fn iterate<T>(&mut self, iters: &mut Vec<T>) -> Option<Self::Item>
    where
        T: Iterator<Item = Self::Item>;
}

pub struct IMExIterCounter<I>
where
    I: IMExIterator,
{
    imex_iter: I,
    counter: u32,
}

use nom::IResult;
pub trait ParserCombinator {
    fn parse(input: &str) -> IResult<&str, Self>
    where
        Self: std::marker::Sized;
}
