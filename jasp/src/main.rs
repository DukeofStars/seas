use jasp::prelude::*;

use std::time::Duration;

fn main() {
    let data = 0..1000;
    let format = ProgressBar::new()
        .link(Percentage::new())
        .add(TimedDisplay::new());
    for _ in data.progress().with_format(format) {
        std::thread::sleep(Duration::from_millis(10));
    }
}
