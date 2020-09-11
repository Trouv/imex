use super::{IMExIterator, IMExVal, ParserCombinator, Quantifier};
use nom::IResult;

/// An [`IMExVal`](./enum.IMExVal.html) that has been quantified, for use in a parsed
/// [`IMEx`](./struct.IMEx.html).
#[derive(PartialEq, Debug, Clone)]
pub struct QuantifiedIMExVal {
    pub val: IMExVal,
    pub quantifier: Quantifier,
    current_val: (Option<IMExVal>, bool),
}

impl QuantifiedIMExVal {
    pub fn from(val: IMExVal, quantifier: Quantifier) -> QuantifiedIMExVal {
        QuantifiedIMExVal {
            current_val: (None, true),
            val,
            quantifier,
        }
    }
}

impl IMExIterator for QuantifiedIMExVal {
    fn iterate<T>(&mut self, iters: &mut Vec<T>) -> Option<Self::Item>
    where
        T: Iterator<Item = Self::Item>,
    {
        loop {
            match &mut self.current_val.0 {
                Some(val) => match val.iterate(iters) {
                    Some(res) => {
                        self.current_val.1 = true;
                        return Some(res);
                    }
                    None => self.current_val.0 = None,
                },
                None => match (self.quantifier.next(), self.current_val.1) {
                    (Some(_), true) => self.current_val = (Some(self.val.clone()), false),
                    _ => return None,
                },
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
