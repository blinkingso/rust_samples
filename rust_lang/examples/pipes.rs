use std::io::prelude::*;
use std::path::Path;
use std::process::{Command, Stdio};

/// rust pipes
fn main() {
    let mut process = match Command::new("minigrep")
        .arg(&Path::new("./"))
        .arg("toml")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Err(e) => panic!("couldn't spawn ls: {}", e),
        Ok(process) => process,
    };

    // write a string to the `stdin` of `ls`.
    // match process.stdin.unwrap().write_all("grep toml".as_bytes()) {
    //     Err(err) => panic!("write error to stdin: {}", err),
    //     Ok(_) => println!("sent pan to ls"),
    // }

    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(err) => panic!("read error : {}", err),
        Ok(_) => println!("ls responded with: \n{}", s),
    }
}
