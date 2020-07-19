use std::io;

pub enum Repeater {
    Infinite,
    Finite(usize),
}

impl Repeater {
    pub fn repeat<F>(&self, mut op: F) -> io::Result<()>
    where
        F: FnMut() -> io::Result<bool>,
    {
        let mut repeat = true;
        let mut rep_count: usize = 0;
        while repeat {
            repeat = op()?;
            rep_count += 1;

            if let Repeater::Finite(x) = self {
                if rep_count >= *x {
                    repeat = false;
                }
            }
        }
        Ok(())
    }
}
