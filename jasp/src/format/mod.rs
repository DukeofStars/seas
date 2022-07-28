use crate::Progress;

pub mod progress_bar;
pub use progress_bar::ProgressBar;
pub mod percentage;
pub use percentage::Percentage;

pub trait ProgressDisplay: Sized + Clone {
    fn display<T: ExactSizeIterator, F: ProgressDisplay>(
        &mut self,
        progress: &Progress<T, F>,
    ) -> String;
}
