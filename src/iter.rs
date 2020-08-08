use crate::zipper::Zipper;
use std::io::Result;

pub trait ZipsIntoOne<T, I>
where
    T: Iterator<Item = I>,
{
    fn zip_into_one_by_zprex(self, iters: &mut Vec<T>, zprex: &str) -> Result<Zipper<T, I>>;

    fn zip_into_one(self, iters: &mut Vec<T>) -> Zipper<T, I>
    where
        Self: std::marker::Sized,
    {
        let iter_count = (iters.len() + 1) as u8;
        let zprex = format!(
            "({})*",
            (0..iter_count).map(|x| x as char).collect::<String>()
        );
        self.zip_into_one_by_zprex(iters, &zprex).unwrap()
    }
}

impl<T, I> ZipsIntoOne<T, I> for T
where
    T: Iterator<Item = I>,
{
    fn zip_into_one_by_zprex(self, iters: &mut Vec<T>, zprex: &str) -> Result<Zipper<T, I>> {
        let mut total_iters = vec![self];
        total_iters.append(iters);
        Zipper::<T, I>::from(total_iters, zprex)
    }
}

trait AllZipsIntoOne<T, I>
where
    T: Iterator<Item = I>,
{
    fn zip_into_one_by_zprex(self, zprex: &str) -> Result<Zipper<T, I>>;
}

impl<T, I> AllZipsIntoOne<T, I> for Vec<T>
where
    T: Iterator<Item = I>,
{
    fn zip_into_one_by_zprex(self, zprex: &str) -> Result<Zipper<T, I>> {
        Zipper::<T, I>::from(self, zprex)
    }
}
