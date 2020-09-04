//! This module contains objects for representing an IMEx, as well as functions for parsing IMExes
//! from strings. There is no logic for performing merges here, just data.
mod imex;
mod imex_val;
pub mod parsers;
mod quantified_imex_val;
mod quantifier;

pub use self::{
    imex::IMEx, imex_val::IMExVal, quantified_imex_val::QuantifiedIMExVal, quantifier::Quantifier,
};

pub trait IMExIterator<T, I>
where
    T: Iterator<Item = I>,
{
    fn iterate(&mut self, iters: &mut Vec<T>) -> Option<I>;
}
