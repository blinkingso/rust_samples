#![allow(unused_variables)]
#![allow(dead_code)]

type File = String;

fn open(f: &mut File) -> bool {
    true
}
fn close(f: &mut File) -> bool {
    true
}

/// '!' tells the compiler the fn never returns;
#[allow(dead_code)]
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
    unimplemented!();
}

fn clear(text: &mut String) {
    *text = String::from("");
}

/// Rust in Action chapter 3;
fn main() {
    let mut f = File::from("f.txt");
    open(&mut f);
    // read(&mut f, &mut vec![]);
    close(&mut f);
}

/// File Description Structure
#[derive(Debug)]
struct FileStruct {
    /// name of a file;
    pub name: String,
    /// full data of a file;
    data: Vec<u8>,
}

impl FileStruct {
    pub fn new(name: &str) -> Self {
        FileStruct {
            name: String::from(name),
            data: vec![],
        }
    }

    fn set_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }

    fn get_data(&self) -> &Vec<u8> {
        &self.data
    }
}

struct Hostname(String);
fn connect(host: Hostname) {
    println!("connected to {}", host.0);
}

#[test]
fn test_hostname() {
    let ordinary_string = String::from("localhost");
    let host = Hostname(ordinary_string.clone());

    connect(host);
}

fn read_file(f: &FileStruct, buf: &mut Vec<u8>) -> usize {
    let mut copy_data = f.get_data().clone();
    let read_len = copy_data.len();
    buf.reserve(read_len);
    buf.append(&mut copy_data);
    read_len
}

#[test]
fn test_read_file() {
    let mut f = FileStruct::new("a.ini");
    let text = "Hello Quxi".as_bytes();
    let mut b = vec![];
    b.append(&mut text.to_vec());
    f.set_data(b);
    let mut buffer = vec![];
    let size = read_file(&f, &mut buffer);
    println!("file size is {}", size);
    let text = String::from_utf8_lossy(&buffer);
    println!("file content is : {}", text);
}
pub(crate) const HELLO_WORLD: &'static str = "hello world";
pub static mut ERROR: i32 = 0;
extern crate rand;
use rand::random;
fn modify_error() -> usize {
    if random() && random() && random() {
        unsafe {
            ERROR = 1;
        }
    }
    0
}

#[test]
fn test_static_const() {
    println!("HELLO_WORLD IS {}", HELLO_WORLD);
    // Accessing and modifying static mut variables requires
    // the use of an unsafe block or function. This is Rust's way of disclaiming all responsibility.
    unsafe {
        println!("ERROR IS : {} BEFORE MODIFY", ERROR);
        ERROR = -1;
        println!("ERROR IS : {} AFTER MODIFY", ERROR);
    }

    // mod error to 1 randomly.
    modify_error();
    unsafe {
        if ERROR == 1 {
            panic!("ERROR occured!!!");
        }
    }
}

enum Suit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

enum Card {
    King(Suit),
    Queen(Suit),
    Jack(Suit),
    Ace(Suit),
    Pip(Suit, usize),
}

/// Defining common behavior with traits;
pub trait Read {
    /// read filedata to temp vec: save_to;
    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}

#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
struct FileOpt {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl FileOpt {
    /// creates a new, empty `FileOpt`
    /// ### Examples
    ///
    /// ```
    /// let f = FileOpt::new("a.txt");
    /// let f = f.open().unwrap();
    /// let f = f.close().unwrap();
    /// ```
    ///
    pub fn new(name: &str) -> FileOpt {
        FileOpt {
            name: name.to_string(),
            data: vec![],
            state: FileState::Closed,
        }
    }

    pub fn new_with_data(name: &str, data: Vec<u8>) -> FileOpt {
        FileOpt {
            name: name.to_string(),
            data,
            state: FileState::Closed,
        }
    }

    fn open(mut self) -> Result<Self, String> {
        self.state = FileState::Open;
        Ok(self)
    }

    fn close(mut self) -> Result<Self, String> {
        self.state = FileState::Closed;
        Ok(self)
    }
}

impl Read for FileOpt {
    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err("File must be open State for reading.".to_string());
        }

        let mut buf = self.data.clone();
        let read_length = buf.len();
        save_to.reserve(read_length);
        save_to.append(&mut buf);
        Ok(read_length)
    }
}

use std::fmt::{self, Display};
impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

impl Display for FileOpt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // do not println data;
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

#[test]
fn test_enum_state() {
    let buf = "Compound data types".as_bytes();
    let file = FileOpt::new_with_data("a.txt", buf.to_vec());
    let file = file.open().unwrap();
    let mut tmp = Vec::with_capacity(buf.len());
    let size = &file.read(&mut tmp).unwrap();
    println!(
        "read file data size is : {}, read data is : {}",
        size,
        String::from_utf8_lossy(&tmp)
    );
    let file = file.close().unwrap();
    // {} Display trait, {:?} Debug trait.
    println!("final file state is : {}", file);
}
