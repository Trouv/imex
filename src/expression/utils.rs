pub trait IMExIterator {
    fn iterate<T, I>(&mut self, iters: &mut Vec<T>) -> Option<I>
    where
        T: Iterator<Item = I>;
}

pub struct IMExIterCounter<T: IMExIterator> {
    imex_iter: T,
    counter: u32,
}

use nom::IResult;
pub trait ParserCombinator {
    fn parse(input: &str) -> IResult<&str, Self>
    where
        Self: std::marker::Sized;
}
