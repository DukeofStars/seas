use colored::Colorize;

use crate::Progress;

pub trait ProgressDisplay: Sized + Clone {
    fn display<T: ExactSizeIterator, F: ProgressDisplay>(
        &mut self,
        progress: &Progress<T, F>,
    ) -> String;
}

#[derive(Clone)]
pub struct JaspFormatter {
    /// The amount of blocks in the progress bar.
    /// eg. [###] would be 3 blocks.
    blocks: usize,
    /// The title of the progress bar.
    title: String,
}

impl Default for JaspFormatter {
    fn default() -> Self {
        Self {
            blocks: 10,
            title: "".to_string(),
        }
    }
}

impl JaspFormatter {
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
}

impl ProgressDisplay for JaspFormatter {
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
