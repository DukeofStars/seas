use super::ProgressDisplay;

#[derive(Clone)]
pub struct LinkedDisplay<T: ProgressDisplay, F: ProgressDisplay>(T, F);

impl<T: ProgressDisplay, F: ProgressDisplay> ProgressDisplay for LinkedDisplay<T, F> {
    fn display<Iter: ExactSizeIterator, X: ProgressDisplay>(
        &mut self,
        progress: &crate::Progress<Iter, X>,
    ) -> String {
        format!("{} {}", self.0.display(progress), self.1.display(progress))
    }
}

pub trait Linkify {
    fn link<New: ProgressDisplay>(self, new: New) -> LinkedDisplay<Self, New>
    where
        Self: ProgressDisplay;
}

impl<Display: ProgressDisplay> Linkify for Display {
    fn link<New: ProgressDisplay>(self, new: New) -> LinkedDisplay<Self, New>
    where
        Self: ProgressDisplay,
    {
        LinkedDisplay(self, new)
    }
}
