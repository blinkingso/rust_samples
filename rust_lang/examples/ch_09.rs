//! time and clock
extern crate chrono;

use chrono::Local;

fn main() {}

/// Encoding time
fn test_encoding_time() {
    let now = Local::now();
    println!("{}", now);
}

/// UNIX timestamps 32-bit integer, represent the milliseconds since epoch. 1970.1.1
struct Time {
    seconds: u32,
    fraction_sec: u32,
}
