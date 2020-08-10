//! A library for merging multiple iterators into one iterator, with the optional use of an IMEx,
//! or Iterator-Merging-Expression, for controlling the merge.
//!
//! # Writing an IMEx
//! IMEx is based off of RegEx. A quick reference of IMEx syntax:
//!  * Digits - indicates the index of the iterator to consume an item from
//!  * `()` - defines a group
//!  * `*` - repeats the previous digit/group until the relevant iterator\(s\) are
//!    exhausted
//!  * `{x}` - repeats the previous digit/group `x` times, or until the relevant
//!    iterator\(s\) are exhausted.
//!
//! ## Example IMExes
//!
//! `"0110"` on two iterators: results in a merged iterator that starts
//! with the first item of the first iterator, then the first two items of the
//! second iterator, then the second item of the first iterator
//!
//! `"(01)*"` on two iterators: results in a merged iterator whose elements
//! alternate between the elements of the input iterators until they are both
//! exhausted.
//!
//! `"(012){4}(122)*"` on three iterators: results in a merged iterator whose
//! elements rotate through the elements of the input iterators 4 times, then picks
//! an element of the second iterator followed by two elements of the third
//! repeatedly until they are both exhausted.
//!
//! # Example Usage
//! ```
//! use ::imex::*;
//!
//! let iter1 = "123456".chars();
//! let iter2 = "abc".chars();
//!
//! let merged = iter1
//!     .imex_merge(iter2, "0(01){4}")
//!     .expect("Invalid IMEx")
//!     .map(|e| e.expect("Index out of range"))
//!     .collect::<String>();
//!
//! assert_eq!(merged, "12a3b4c5");
//! ```
//! There are alternatives for [`imex_merge`](merges/trait.IMExMerges.html#method.imex_merge) for
//! merging more than 2 iterators or using the default alternating/rotating IMEx. These functions
//! can be found here:
//!  * [`imex_merge_all`](merges/trait.IMExMerges.html#method.imex_merge_all)
//!  * [`rot_merge_all`](merges/trait.IMExMerges.html#method.rot_merge_all)
//!  * [`alt_merge`](merges/trait.IMExMerges.html#method.alt_merge)

pub mod imex;
pub mod iter;
pub mod merges;
pub mod quantifier;

pub use merges::IMExMerges;
