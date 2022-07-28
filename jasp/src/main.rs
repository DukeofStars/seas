use jasp::prelude::*;

use std::time::Duration;

fn main() {
    let data = 0..100;
    let format = ProgressBar::new();
    for _ in data.progress().with_format(format) {
        std::thread::sleep(Duration::from_millis(10));
    }
}
