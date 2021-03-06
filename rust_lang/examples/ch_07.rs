//! ch07: Files and storage;
//! How data is represented on physical storage devices;
//! Writing data structures to your preferred file format;
//! Building a tool to read from a file and inspect its contents;
//! Creating a working k-v store that's immune from corruption;
extern crate bincode;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use serde_derive::Serialize;

#[derive(Serialize)]
struct City {
    name: String,
    population: usize,
    latitude: f64,
    longitude: f64,
}

fn main() {
    let city = City {
        name: String::from("shh"),
        population: 1000000,
        latitude: 1.0,
        longitude: 1.11,
    };
    let _vec = serde_json::to_vec(&city).unwrap();
    println!("jsonbinary: {:?}, len is : {}", _vec, _vec.len());
    let _bincode = bincode::serialize(&city).unwrap();
    println!("bincode: {:?}", _bincode);

    let mut position_in_input = 0;
    for line in _vec.chunks(16) {
        print!("[0x{:08x}] ", position_in_input);
        for byte in line {
            print!("{:02x} ", byte);
        }

        println!();
        position_in_input += 16;
    }

    f_view(&Path::new("./ch_06.rs")).unwrap();
}
const BYTES_PER_LINE: usize = 16;
use std::boxed::Box;
use std::error::Error;
use std::fs::{File, OpenOptions};
fn f_view(f: &Path) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(f)?;
    let mut pos = 0;
    let mut buffer = [0; BYTES_PER_LINE];
    while let Ok(_) = f.read_exact(&mut buffer) {
        print!("[0x{:08x}] ", pos);
        for byte in &buffer {
            match *byte {
                0x00 => print!(".   "),
                0xff => print!("##  "),
                _ => print!("{:02x} ", byte),
            }
        }
        println!("");
        pos += BYTES_PER_LINE;
    }
    Ok(())
}

#[test]
fn test_file_ops() -> Result<(), Box<dyn Error>> {
    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .append(false)
        .create(true)
        .truncate(true)
        .open("test_file_ops.test")?;
    let mut buffer = [0; 256];
    let mut src = File::open("examples/ch_07.rs")?;
    while let Ok(size) = src.read(&mut buffer) {
        if size == 0 {
            break;
        }
        println!("read : {} bytes.", size);
        f.write_all(&buffer[..size])?;
    }
    f.flush()?;
    println!("file rewrite success!!!");

    Ok(())
}

#[test]
/// Interacting with the filesystem in a type-safe manner with `std::fs::Path`
fn test_fs_path() {
    let path = "/tmp/hello.txt";
    let dir = path.split('/').nth(1);
    println!("dir is : {:?}", dir);
    let mut _path = PathBuf::from(path);
    _path.pop();
    println!("dir is : {}", _path.display());
}

fn basic_hash(key: &str) -> u32 {
    let first = key.chars().next().unwrap_or('\0');
    unsafe { std::mem::transmute::<char, u32>(first) }
}
use std::collections::HashMap;
#[test]
fn test_hash() {
    let mut capitals = HashMap::new();
    capitals.insert("Fiji", "suva");
    let suva = capitals.get("Fiji").unwrap_or(&"None");
    // will panic here if `Fiji` key not exists in `capitals`
    let suva = capitals["Fiji"];
    println!("suva : {}", suva);
}
