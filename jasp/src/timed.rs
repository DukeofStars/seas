use crate::{format::ProgressDisplay, Progress};

#[derive(Clone)]
pub struct TimedFormatter<T: ProgressDisplay> {
    last_time: std::time::Instant,
    formatter: T,
}

impl<Inner: ProgressDisplay> ProgressDisplay for TimedFormatter<Inner> {
    fn display<T: ExactSizeIterator, F: ProgressDisplay>(
        &mut self,
        progress: &Progress<T, F>,
    ) -> String {
        // Get the time it takes from the last call to `display` to now.
        let time_since_last_call = self.last_time.elapsed();
        self.last_time = std::time::Instant::now();
        // Calculate the time per item.
        let time_per_item = time_since_last_call.as_secs() as f64 / progress.i as f64;
        // Calculate the time remaining.
        let time_remaining = (progress.len - progress.i) as f64 * time_per_item;
        // Format the time remaining.
        let time_remaining_str = format!("{:.2}s", time_remaining.round() as u64);
        // Format the progress bar.
        format!(
            "{} ETA {}   ",
            self.formatter.display(progress),
            time_remaining_str
        )
    }
}

/// Creates a new `TimedFormatter` with the given `ProgressDisplay`.
pub trait TimedFormatterExt {
    fn timed(self) -> TimedFormatter<Self>
    where
        Self: ProgressDisplay;
}

impl<T: ProgressDisplay> TimedFormatterExt for T {
    fn timed(self) -> TimedFormatter<Self> {
        TimedFormatter {
            last_time: std::time::Instant::now(),
            formatter: self,
        }
    }
}
