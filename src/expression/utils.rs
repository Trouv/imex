pub trait IMExIterator {
    fn iterate<T, I>(&mut self, iters: &mut Vec<T>) -> Option<I>
    where
        T: Iterator<Item = I>;
}

#[derive(PartialEq, Debug, Clone)]
pub struct IMExIterCounter<X: IMExIterator> {
    imex_iter: X,
    counter: u32,
}

impl<X: IMExIterator> IMExIterator for IMExIterCounter<X> {
    fn iterate<T, I>(&mut self, iters: &mut Vec<T>) -> Option<I>
    where
        T: Iterator<Item = I>,
    {
        match self.imex_iter.iterate(iters) {
            Some(res) => {
                self.counter += 1;
                Some(res)
            }
            None => None,
        }
    }
}

impl<X: IMExIterator> IMExIterCounter<X> {
    pub fn new(imex_iter: X) -> IMExIterCounter<X> {
        IMExIterCounter {
            imex_iter,
            counter: 0,
        }
    }

    pub fn count(&self) -> u32 {
        self.counter
    }
}

use nom::IResult;
pub trait ParserCombinator {
    fn parse(input: &str) -> IResult<&str, Self>
    where
        Self: std::marker::Sized;
}
