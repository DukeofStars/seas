use super::ProgressDisplay;

#[derive(Clone, Default)]
pub struct Percentage;

impl Percentage {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ProgressDisplay for Percentage {
    fn display<T: ExactSizeIterator, F: ProgressDisplay>(
        &mut self,
        progress: &crate::Progress<T, F>,
    ) -> String {
        let percentage: f32 = progress.i as f32 / progress.len as f32 * 100 as f32;
        format!("{:.1}%", percentage)
    }
}
