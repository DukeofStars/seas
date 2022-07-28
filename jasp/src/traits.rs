use crate::{format::ProgressBar, Progress};

pub trait Progressify: Sized + ExactSizeIterator {
    fn progress(self) -> Progress<Self, ProgressBar>;
}

impl<Iter: ExactSizeIterator> Progressify for Iter {
    fn progress(self) -> Progress<Self, ProgressBar> {
        Progress::new(self)
    }
}
