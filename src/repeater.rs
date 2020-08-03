use std::io;

#[derive(PartialEq, Debug)]
pub enum Repeater {
    Infinite,
    Finite(usize),
}

trait Repeator {
    type Item;

    fn once(&mut self) -> Option<Self::Item>;

    fn get_repeater(&mut self) -> Repeater;
}

impl<I> Iterator for dyn Repeator<Item = I> {
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        if let Repeater::Finite(i) = self.get_repeater() {
            if i > 0 {}
        }
        None
    }
}

impl Repeater {
    pub fn repeat<F>(&self, mut op: F) -> io::Result<()>
    where
        F: FnMut() -> io::Result<bool>,
    {
        let mut repeat = true;
        let mut rep_count: usize = 0;
        while repeat {
            if let Repeater::Finite(x) = self {
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
    fn infinite_repeater_repeats_until_false() {
        let r = Repeater::Infinite;
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
        let r = Repeater::Infinite;
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
        let r = Repeater::Finite(10);
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
        let r = Repeater::Finite(10);
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
        let r = Repeater::Finite(10);
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
        let r = Repeater::Finite(0);
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
