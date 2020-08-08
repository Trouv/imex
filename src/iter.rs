use crate::zipper::Zipper;
use std::io::Result;

pub trait ZipsIntoOne {
    type Item;
    fn zip_into_one(
        self: Box<Self>,
        iters: &mut Vec<Box<dyn Iterator<Item = Self::Item>>>,
        zprex: &str,
    ) -> Result<Zipper<Self::Item>>;
}

impl<I> ZipsIntoOne for dyn Iterator<Item = I> {
    type Item = I;
    fn zip_into_one(
        self: Box<Self>,
        iters: &mut Vec<Box<dyn Iterator<Item = Self::Item>>>,
        zprex: &str,
    ) -> Result<Zipper<Self::Item>> {
        let mut total_iters: Vec<Box<dyn Iterator<Item = Self::Item>>> = vec![self];
        total_iters.append(iters);
        Zipper::<Self::Item>::from(total_iters, zprex)
    }
}

trait AllZipsIntoOne {
    type Item;
    fn zip_into_one(self, zprex: &str) -> Result<Zipper<Self::Item>>;
}

impl<I> AllZipsIntoOne for Vec<Box<dyn Iterator<Item = I>>> {
    type Item = I;
    fn zip_into_one(self, zprex: &str) -> Result<Zipper<Self::Item>> {
        Zipper::<Self::Item>::from(self, zprex)
    }
}
