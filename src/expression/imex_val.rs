use super::{IMEx, IMExIterator};
use std::iter::Once;

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

impl<T, I> IMExIterator<T, I> for IMExVal
where
    T: Iterator<Item = I>,
{
    fn iterate(&mut self, iters: &mut Vec<T>) -> Option<I> {
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
