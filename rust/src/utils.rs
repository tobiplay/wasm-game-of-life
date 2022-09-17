extern crate web_sys;
use web_sys::console;

/// A timer that can be used to measure the time between two events.
///
/// Each timer has a `name`, which is used to identify it in the console.
/// The data is printed to the console when the timer is dropped.
pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    /// Create a new timer with the given `name`.
    ///
    /// Each timer has a label and returns a `Timer` struct
    /// with this correct label.
    pub fn new(name: &'a str) -> Timer<'a> {
        console::time_with_label(name);
        Timer { name }
    }
}

// We'll implement the `Drop` trait for our `Timer` struct.
// This will ensure that the `timeEnd` JavaScript function is called
// when the timer goes out of scope.
impl<'a> Drop for Timer<'a> {
    /// Drops the timer and print the time to the console.
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
