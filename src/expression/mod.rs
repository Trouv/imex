//! This module contains objects for representing an IMEx, as well as functions for parsing IMExes
//! from strings. There is no logic for performing merges here, just data.
pub mod imex;
pub mod parsers;
pub mod quantifier;

pub use self::{
    imex::{IMEx, IMExVal, QuantifiedIMExVal},
    quantifier::Quantifier,
};
