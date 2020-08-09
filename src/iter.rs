use crate::zipper::Zipper;
use std::io::Result;

pub trait ZipsIntoOne<T, I>
where
    T: Iterator<Item = I>,
{
    fn zip_into_one_by_zprex(self, iters: &mut Vec<T>, zprex: &str) -> Result<Zipper<T, I>>;

    fn zip_into_one(self, iters: &mut Vec<T>) -> Zipper<T, I>
    where
        Self: Sized,
    {
        let iter_count = (iters.len() + 1) as u8;
        let zprex = format!(
            "({})*",
            (0..iter_count).map(|x| x as char).collect::<String>()
        );
        self.zip_into_one_by_zprex(iters, &zprex)
            .expect("Default zprex should have been valid, but wasn't")
    }

    fn zip_both_into_one_by_zprex(self, other: T, zprex: &str) -> Result<Zipper<T, I>>
    where
        Self: Sized,
    {
        self.zip_into_one_by_zprex(&mut vec![other], zprex)
    }

    fn zip_both_into_one(self, other: T) -> Zipper<T, I>
    where
        Self: Sized,
    {
        self.zip_both_into_one_by_zprex(other, "(01)*")
            .expect("Default zprex should have been valid, but wasn't")
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
