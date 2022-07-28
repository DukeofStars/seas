use crate::Progress;

pub mod linked;
pub use linked::*;
pub mod progress_bar;
pub use progress_bar::*;
pub mod percentage;
pub use percentage::*;
pub mod timed;
pub use timed::*;

pub trait ProgressDisplay: Sized + Clone {
    fn display<T: ExactSizeIterator, F: ProgressDisplay>(
        &mut self,
        progress: &Progress<T, F>,
    ) -> String;
}
