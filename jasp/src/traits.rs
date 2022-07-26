use crate::{format, Progress};

pub trait ProgressExt: Sized + ExactSizeIterator {
    fn progress(self) -> Progress<Self, format::JaspFormatter>;
}

impl<Iter: ExactSizeIterator> ProgressExt for Iter {
    fn progress(self) -> Progress<Self, format::JaspFormatter> {
        Progress::new(self)
    }
}
