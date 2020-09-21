use crate::{
    expression::{IMExIterCounter, IMExVal, ParserCombinator, Quantifier},
    IMExIterator,
};
use nom::IResult;

/// An [`IMExVal`](./enum.IMExVal.html) that has been quantified, for use in a parsed
/// [`IMEx`](./struct.IMEx.html).
#[derive(PartialEq, Debug, Clone)]
pub struct QuantifiedIMExVal {
    val: IMExVal,
    quantifier: Quantifier,
    current_val: Option<IMExIterCounter<IMExVal>>,
}

impl QuantifiedIMExVal {
    pub fn new(val: IMExVal, quantifier: Quantifier) -> QuantifiedIMExVal {
        QuantifiedIMExVal {
            val,
            quantifier,
            current_val: None,
        }
    }
}

impl QuantifiedIMExVal {
    /// If self.quantifier isn't depleted, repeats self.val (clones it to self.current_val) and
    /// returns true. Otherwise, returns false.
    fn repeat(&mut self) -> bool {
        match self.quantifier.next() {
            Some(_) => {
                self.current_val = Some(IMExIterCounter::new(self.val.clone()));
                true
            }
            None => false,
        }
    }

    /// Returns an immutable reference to self.val
    pub fn get_val(&self) -> &IMExVal {
        &self.val
    }

    /// Returns an immutable reference to self.quantifier
    pub fn get_quantifier(&self) -> &Quantifier {
        &self.quantifier
    }
}

impl IMExIterator for QuantifiedIMExVal {
    fn iterate<T, I>(&mut self, iters: &mut Vec<T>) -> Option<I>
    where
        T: Iterator<Item = I>,
    {
        loop {
            match &mut self.current_val {
                Some(val) => match val.iterate(iters) {
                    Some(res) => return Some(res),
                    None => {
                        if !(val.count() > 0 && self.repeat()) {
                            return None;
                        }
                    }
                },
                None => {
                    if !self.repeat() {
                        return None;
                    }
                }
            }
        }
    }
}

impl ParserCombinator for QuantifiedIMExVal {
    fn parse(input: &str) -> IResult<&str, QuantifiedIMExVal> {
        let (input, val) = IMExVal::parse(input)?;
        let (input, quantifier) = Quantifier::parse(input)?;
        Ok((input, QuantifiedIMExVal::new(val, quantifier)))
    }
}

#[cfg(test)]
mod tests {
    use super::super::IMEx;
    use super::*;
    use std::{convert::TryFrom, io::Result, iter::once};

    #[test]
    fn iterating_new_qimex_val_with_zero_quantifier_gives_none() {
        let mut qimex_val = QuantifiedIMExVal::new(IMExVal::Single(once(1)), Quantifier::Finite(0));
        let mut iters = vec!["123".chars(), "abc".chars()];

        assert_eq!(qimex_val.iterate(&mut iters), None);
    }

    #[test]
    fn iterating_qimex_val_with_three_quantifier_repeats_thrice() {
        let mut qimex_val = QuantifiedIMExVal::new(IMExVal::Single(once(1)), Quantifier::Finite(3));
        let mut iters = vec!["123".chars(), "abcde".chars()];

        assert_eq!(qimex_val.iterate(&mut iters), Some('a'));
        assert_eq!(qimex_val.iterate(&mut iters), Some('b'));
        assert_eq!(qimex_val.iterate(&mut iters), Some('c'));
        assert_eq!(qimex_val.iterate(&mut iters), None);
    }

    #[test]
    fn exhausting_imex_val_before_repeats_copmlete_gives_none() {
        let mut qimex_val = QuantifiedIMExVal::new(IMExVal::Single(once(1)), Quantifier::Finite(5));
        let mut iters = vec!["123".chars(), "abc".chars()];

        assert_eq!(qimex_val.iterate(&mut iters), Some('a'));
        assert_eq!(qimex_val.iterate(&mut iters), Some('b'));
        assert_eq!(qimex_val.iterate(&mut iters), Some('c'));
        assert_eq!(qimex_val.iterate(&mut iters), None);
    }

    #[test]
    fn group_imex_val_completes_inner_iteration_first() -> Result<()> {
        let mut qimex_val = QuantifiedIMExVal::new(
            IMExVal::Group(IMEx::try_from("01*")?),
            Quantifier::Finite(2),
        );
        let mut iters = vec!["123".chars(), "abc".chars()];

        assert_eq!(qimex_val.iterate(&mut iters), Some('1'));
        assert_eq!(qimex_val.iterate(&mut iters), Some('a'));
        assert_eq!(qimex_val.iterate(&mut iters), Some('b'));
        assert_eq!(qimex_val.iterate(&mut iters), Some('c'));
        assert_eq!(qimex_val.iterate(&mut iters), Some('2'));
        assert_eq!(qimex_val.iterate(&mut iters), None);

        Ok(())
    }
}
