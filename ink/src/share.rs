// Contains global variables to be shared between the ui thread and the worker thread.
#[derive(Default)]
pub struct Global {
    pub percentage: f64,
    pub should_close: bool,
}
