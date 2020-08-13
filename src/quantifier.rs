use std::io::{Error, ErrorKind::InvalidInput, Result};

/// Represents a quantifier in a parsed [`IMEx`](../imex/struct.IMEx.html). Either Finite (`{x}`), in
/// which case a range is contained, or Infinite (`*`).
#[derive(PartialEq, Debug, Clone)]
pub enum Quantifier {
    Infinite,
    Finite(usize),
}

impl Iterator for Quantifier {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        match *self {
            Quantifier::Finite(0) => None,
            Quantifier::Finite(n) => {
                *self = Quantifier::Finite(n - 1);
                Some(())
            }
            _ => Some(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_quantifier_iterates_thrice() {
        let mut r = Quantifier::Finite(3);
        assert_eq!(r.next(), Some(()));
        assert_eq!(r, Quantifier::Finite(2));
        assert_eq!(r.next(), Some(()));
        assert_eq!(r, Quantifier::Finite(1));
        assert_eq!(r.next(), Some(()));
        assert_eq!(r, Quantifier::Finite(0));
        assert_eq!(r.next(), None);
    }

    #[test]
    fn zero_quantifier_gives_immediate_none() {
        let mut r = Quantifier::Finite(0);
        assert_eq!(r.next(), None);
    }

    #[test]
    fn infinite_quantifier_iterates_a_lot() {
        let mut r = Quantifier::Infinite;
        for _ in 0..100 {
            assert_eq!(r.next(), Some(()));
        }
    }
}
