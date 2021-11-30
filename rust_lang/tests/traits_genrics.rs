use std::ops::Add;
use std::process::Output;
use std::vec::IntoIter;
use std::{collections::HashMap, fmt::Debug, hash::Hash, io::Write, iter::Cycle};

#[test]
fn test_trait_genrics() {
    use std::boxed::Box;
    use std::io::Write;
    let mut buf: Vec<u8> = vec![];
    // let mut writer: Box<dyn Write> = Box::new(buf);
    // writer.write("hello".as_bytes());
    let writer: &mut dyn Write = &mut buf;
}
struct Canvas;

trait Visible {
    fn draw(&self, canvas: &mut Canvas);
    fn hit_test(&self, x: i32, y: i32) -> bool;
}

struct Broom;
impl Visible for Broom {
    fn draw(&self, canvas: &mut Canvas) {
        println!("draw broom");
    }

    fn hit_test(&self, x: i32, y: i32) -> bool {
        println!("hit test");
        true
    }
}

/// A `std::io::Write` that ignores whatever data you write to it.
pub struct Sink;

impl std::io::Write for Sink {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[test]
fn test_sink() {
    let mut out = Sink;
    let _ = out.write_all(b"hello world").unwrap();
}

trait IsEmoji {
    /// check the built-in character type
    fn is_emoji(&self) -> bool;
}

impl IsEmoji for char {
    fn is_emoji(&self) -> bool {
        if self.is_numeric() || self.is_ascii() || self.is_whitespace() {
            return false;
        }

        true
    }
}

use graphics::Viewport;
use serde::Serialize;
use std::fs::File;
pub struct SerializeToFile(String);
impl SerializeToFile {
    pub fn save_to_file<Conf>(self, config: Conf) -> std::io::Result<()>
    where
        Conf: Serialize,
    {
        let file_name = format!("{}.json", self.0);
        let file = File::create(&file_name).unwrap();
        let mut serializer = serde_json::Serializer::new(file);
        config.serialize(&mut serializer);

        Ok(())
    }
}

#[test]
fn test_serialize() {
    let utils = SerializeToFile(String::from("user"));
    let mut map = HashMap::new();
    map.insert("name", "yaphets");
    map.insert("age", "10");
    map.insert("email", "yaphets.andrew@gmail.com");
    let _ = utils.save_to_file(map);
}

pub trait Clonable {
    fn clone(&self) -> Self;
}

trait Spliceable {
    fn splice(&self, rhs: &Self) -> Self;
}
struct CherryTree;
struct Mammoth;
impl Spliceable for CherryTree {
    fn splice(&self, rhs: &Self) -> Self {
        CherryTree
    }
}

impl Spliceable for Mammoth {
    fn splice(&self, rhs: &Self) -> Self {
        Mammoth
    }
}

/// `Spliceable` cannot be made into an object
// fn splice_anything(left: &dyn Spliceable, right: &dyn Spliceable) {
//     let combo = left.splice(right);
//     print!("now is ok!");
// }

trait MegaSplice {
    fn splice(&self, rhs: &dyn MegaSplice) -> Box<dyn MegaSplice>;
}

fn splice_anything_mega(left: &dyn MegaSplice, right: &dyn MegaSplice) {
    let combo = left.splice(right);
    print!("now is ok!");
}

struct L;
struct R;
impl MegaSplice for L {
    fn splice(&self, rhs: &dyn MegaSplice) -> Box<dyn MegaSplice> {
        Box::new(L)
    }
}

impl MegaSplice for R {
    fn splice(&self, rhs: &dyn MegaSplice) -> Box<dyn MegaSplice> {
        Box::new(R)
    }
}

#[test]
fn test_splice() {
    // splice_anything(&Mammoth, &CherryTree);
    splice_anything_mega(&L, &R);
}

type Direction = u8;

trait Creature: Visible {
    fn position(&self) -> (i32, i32);
    fn facing(&self) -> Direction;
}

struct Creatural;
impl Creature for Creatural {
    fn position(&self) -> (i32, i32) {
        (0, 0)
    }

    fn facing(&self) -> Direction {
        1u8
    }
}

impl Visible for Creatural {
    fn draw(&self, canvas: &mut Canvas) {
        println!("Visible draw");
    }

    fn hit_test(&self, x: i32, y: i32) -> bool {
        true
    }
}

#[test]
fn test_sub_trait() {
    let c = Creatural;
    c.draw(&mut Canvas);
    println!(
        "pos: {:?}, facing: {}, hit_test: {}",
        c.position(),
        c.facing(),
        c.hit_test(0, 0)
    )
}

trait StringSet {
    /// Return a new empty set.
    fn new() -> Self
    where
        Self: Sized;
    /// Return a set that contains all the strings in `strings`.
    fn from_slice(string: &[&str]) -> Self;
    /// Find out if this set contains a particular `value`.
    fn contains(&self, string: &str) -> bool;
    /// Add a string to this set.
    fn add(&mut self, string: &str);
}

/// Return the set of words in `document` that aren't in `worldlist`.
fn unknown_words<S: StringSet>(document: &[String], wordlist: &S) -> S {
    let mut unknowns = S::new();
    for word in document {
        if !wordlist.contains(word) {
            unknowns.add(word)
        }
    }

    unknowns
}

#[test]
fn test_str() {
    let str = ToString::to_string("hello");
    let str2 = str::to_string("hello");
    // str type ===> &str for &self form
    println!("str: {}, str2: {}", str, str2);
    let zero = 0;
    i64::abs(zero);

    let words = "java android c++ c python go".split_whitespace();
    for (idx, value) in words.into_iter().enumerate() {
        println!("words[{}] = {}", idx, value);
    }
}

/// How iterators work.
pub trait Iterator2 {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

fn collect_into_vector<I: Iterator>(iter: I) -> Vec<I::Item> {
    let mut results = Vec::new();
    for value in iter {
        results.push(value);
    }
    results
}

fn dump<I>(iter: I)
where
    I: Iterator,
    I::Item: Debug,
{
    println!("======start to dump=======");
    for value in iter {
        println!("value:\t{:?}", &value);
    }
    println!("=======end to dump========");
}

#[test]
fn test_dump() {
    let to_dump = "java js css html android rust ruby".split_whitespace();
    dump(to_dump.into_iter());
}

pub trait MulDe<RHS> {
    const MAX_PORT: u32 = 65535;
    type Output;
    fn mul(self, rhs: RHS) -> Self::Output;
}

use std::iter::Chain;
fn cyclical_zip(v: Vec<u8>, u: Vec<u8>) -> Cycle<Chain<IntoIter<u8>, IntoIter<u8>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}

/// impl traits without dynamic dispatch or a heap allocation.
fn cyclical_zip2(v: Vec<u8>, u: Vec<u8>) -> impl Iterator<Item = u8> {
    v.into_iter().chain(u.into_iter()).cycle()
}

trait Float {
    const ZERO: Self;
    const ONE: Self;
}

impl Float for f32 {
    const ZERO: f32 = 0.0;
    const ONE: f32 = 1.0;
}

impl Float for f64 {
    const ZERO: f64 = 0.0;
    const ONE: f64 = 1.0;
}

fn add_one<T>(value: T) -> T
where
    T: Float + Add<Output = T>,
{
    value + T::ONE
}

impl Float for usize {
    const ZERO: usize = 0;
    const ONE: usize = 1;
}

#[test]
fn test_add_one() {
    let f = 10.1;
    let res = add_one(f);
    println!("add one res is : {}", res);
}

fn fib<T>(n: usize) -> T
where
    T: Float + Add<Output = T>,
{
    match n {
        0 => T::ZERO,
        1 => T::ONE,
        n => fib::<T>(n - 1) + fib::<T>(n - 2),
    }
}

#[test]
fn test_fib() {
    println!("fib(5) is : {}", fib::<usize>(5));
}
