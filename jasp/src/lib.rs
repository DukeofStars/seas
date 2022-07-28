use std::{io::Write, iter::ExactSizeIterator};

use format::{ProgressBar, ProgressDisplay};

pub mod format;
pub mod prelude;
pub mod timed;
mod traits;

// Testing
#[cfg(test)]
mod tests;

// Core
pub struct Progress<T: ExactSizeIterator, F: format::ProgressDisplay> {
    iter: T,
    i: usize,
    pub format: F,
    pub len: usize,
}

impl<T: ExactSizeIterator> Progress<T, ProgressBar> {
    pub fn new(iter: T) -> Self {
        Self {
            len: iter.len(),
            iter,
            i: 0,
            format: ProgressBar::default(),
        }
    }
}

impl<T: ExactSizeIterator, F: ProgressDisplay> Progress<T, F> {
    pub fn with_format<U: ProgressDisplay>(self, format: U) -> Progress<T, U> {
        Progress {
            iter: self.iter,
            i: self.i,
            format,
            len: self.len,
        }
    }

    fn draw(&mut self) -> String {
        let mut format = self.format.clone();
        format!("{}", format.display(self))
    }
}

impl<T: ExactSizeIterator, F: format::ProgressDisplay> Iterator for Progress<T, F> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        print!("\r{}", self.draw());
        std::io::stdout().flush().unwrap();
        self.i += 1;

        let res = self.iter.next();
        if res.is_none() {
            println!();
        }
        res
    }
}
