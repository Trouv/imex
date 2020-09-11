use super::{IMExIterCounter, IMExIterator, IMExVal, ParserCombinator, Quantifier};
use nom::IResult;

/// An [`IMExVal`](./enum.IMExVal.html) that has been quantified, for use in a parsed
/// [`IMEx`](./struct.IMEx.html).
#[derive(PartialEq, Debug, Clone)]
pub struct QuantifiedIMExVal {
    pub val: IMExVal,
    pub quantifier: Quantifier,
    current_val: Option<IMExIterCounter<IMExVal>>,
}

impl QuantifiedIMExVal {
    pub fn from(val: IMExVal, quantifier: Quantifier) -> QuantifiedIMExVal {
        QuantifiedIMExVal {
            val,
            quantifier,
            current_val: None,
        }
    }
}

impl QuantifiedIMExVal {
    fn repeat(&mut self) -> bool {
        match self.quantifier.next() {
            Some(_) => {
                self.current_val = Some(IMExIterCounter::new(self.val.clone()));
                true
            }
            None => false,
        }
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
        Ok((input, QuantifiedIMExVal::from(val, quantifier)))
    }
}
