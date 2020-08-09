use std::io;

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

impl Quantifier {
    pub fn repeat<F>(&self, mut op: F) -> io::Result<()>
    where
        F: FnMut() -> io::Result<bool>,
    {
        let mut repeat = true;
        let mut rep_count: usize = 0;
        while repeat {
            if let Quantifier::Finite(x) = self {
                if rep_count >= *x {
                    break;
                }
            }

            repeat = op()?;
            rep_count += 1;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeater_iter_test() {
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
    fn infinite_repeater_repeats_until_false() {
        let r = Quantifier::Infinite;
        let mut c = 0;
        r.repeat(|| -> io::Result<bool> {
            if c < 100 {
                c += 1;
                Ok(true)
            } else {
                Ok(false)
            }
        })
        .unwrap();

        assert_eq!(c, 100);
    }

    #[test]
    fn infinite_repeater_runs_once_for_immediate_false() {
        let r = Quantifier::Infinite;
        let mut c = 0;
        r.repeat(|| -> io::Result<bool> {
            c += 1;
            Ok(false)
        })
        .unwrap();

        assert_eq!(c, 1);
    }

    #[test]
    fn finite_repeater_repeats_until_end_of_range() {
        let r = Quantifier::Finite(10);
        let mut c = 0;
        r.repeat(|| -> io::Result<bool> {
            if c < 20 {
                c += 1;
                Ok(true)
            } else {
                Ok(false)
            }
        })
        .unwrap();

        assert_eq!(c, 10);
    }

    #[test]
    fn finite_repeater_repeats_until_false() {
        let r = Quantifier::Finite(10);
        let mut c = 0;
        r.repeat(|| -> io::Result<bool> {
            if c < 5 {
                c += 1;
                Ok(true)
            } else {
                Ok(false)
            }
        })
        .unwrap();

        assert_eq!(c, 5);
    }

    #[test]
    fn finite_repeater_runs_once_for_immediate_false() {
        let r = Quantifier::Finite(10);
        let mut c = 0;
        r.repeat(|| -> io::Result<bool> {
            c += 1;
            Ok(false)
        })
        .unwrap();

        assert_eq!(c, 1);
    }

    #[test]
    fn finite_repeater_doesnt_run_for_zero_range() {
        let r = Quantifier::Finite(0);
        let mut c = 0;
        r.repeat(|| -> io::Result<bool> {
            if c < 5 {
                c += 1;
                Ok(true)
            } else {
                Ok(false)
            }
        })
        .unwrap();

        assert_eq!(c, 0);
    }
}
