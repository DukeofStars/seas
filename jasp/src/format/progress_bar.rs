use colored::Colorize;

use crate::Progress;

use super::ProgressDisplay;

#[derive(Clone)]
pub struct ProgressBar {
    /// The amount of blocks in the progress bar.
    /// eg. [###] would be 3 blocks.
    blocks: usize,
    /// The title of the progress bar.
    title: String,
}

impl Default for ProgressBar {
    fn default() -> Self {
        Self {
            blocks: 10,
            title: "".to_string(),
        }
    }
}

impl ProgressBar {
    /// Sets the amount of blocks in the progress bar.
    /// eg. [###] would be 3 blocks.
    pub fn with_blocks(mut self, blocks: usize) -> Self {
        self.blocks = blocks;
        self
    }
    /// Sets the title of the progress bar.
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string() + " ";
        self
    }

    /// Creates a new progress bar.
    pub fn new() -> Self {
        Self::default()
    }
}

impl ProgressDisplay for ProgressBar {
    fn display<T: ExactSizeIterator, F: ProgressDisplay>(
        &mut self,
        progress: &Progress<T, F>,
    ) -> String {
        let filled_blocks = (progress.i * self.blocks) / progress.len;
        format!(
            "{}[{}{}]",
            self.title,
            "#".repeat(filled_blocks).blue(),
            " ".repeat(self.blocks - filled_blocks)
        )
    }
}
