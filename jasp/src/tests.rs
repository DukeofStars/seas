use colored::Colorize;

use crate::prelude::*;

#[test]
fn test_draw() {
    let data = 0..10;
    let mut progress = data.progress();
    assert_eq!(progress.draw(), format!("[{}          ]", "".blue()));
    progress.next();
    assert_eq!(progress.draw(), format!("[{}         ]", "#".blue()));
}
