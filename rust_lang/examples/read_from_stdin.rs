#![allow(unused)]
use std::io::{self, BufRead};

/// read data from stdin
fn main() {
    println!("start to read from stdin, please enter: ");
    let mut buf = String::new();
    let size = io::stdin()
        .lock()
        .read_line(&mut buf)
        .expect("read error from terminal");
    println!("buf is : {}", buf);
    println!("read {} bufferd elements", size);
}
