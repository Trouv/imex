pub trait IMExIterator<T, I>
where
    T: Iterator<Item = I>,
{
    fn iterate(&mut self, iters: &mut Vec<T>) -> Option<I>;
}

use nom::IResult;
pub trait ParserCombinator {
    fn parse(input: &str) -> IResult<&str, Self>
    where
        Self: std::marker::Sized;
}
