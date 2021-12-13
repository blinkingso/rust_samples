//! Iterators;

use std::iter::FromIterator;
use std::{fmt::Debug, iter::from_fn};

fn triangle(n: i32) -> i32 {
    (1..=n).fold(0, |sum, item| sum + item)
}

#[test]
fn test_triangle() {
    let res = triangle(10);
    println!("result is : {}", res);
}

pub struct Count(u32);
impl Iterator for Count {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        } else {
            let result = Some(self.0);
            self.0 -= 1;
            return result;
        }
    }
}

pub struct OddNumbers(usize);
impl Iterator for OddNumbers {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 % 2 == 0 {
            println!("not a odd number: {}", self.0);
            return None;
        } else {
            if self.0 < 1 {
                return None;
            } else if self.0 == 1 {
                self.0 = 0;
                return Some(1);
            } else {
                let odd = self.0;
                // check and set.
                let sub = self.0.checked_sub(2);
                if let None = sub {
                    return None;
                } else {
                    self.0 = sub.unwrap();
                }
                return Some(odd);
            }
        }
    }
}

#[test]
fn test_numbers() {
    let counts = Count(100);
    let sum = counts.sum::<u32>();
    println!("totals is : {}", sum);
    {
        let odds = OddNumbers(101);
        let counts2 = Count(50);
        println!("size: {}, {}", odds.count(), counts2.count());
    }

    let odds = OddNumbers(101);
    let counts2 = Count(2);
    let odds3 = odds
        .zip(counts2)
        .map(|(x, y)| x + y as usize)
        .collect::<Vec<usize>>();
    let mut c = 0;
    for odd in &odds3 {
        println!("=> {}", odd);
        c += 1;
    }
    println!("elements: {}", c);
}

#[test]
fn test_slice() {
    let mut data = [1, 2, 3, 4, 5];
    let s = &mut data[0..3];
    s[0] = 5;
    println!("{:?}", data);

    let mut d = data.iter();
    assert_eq!(Some(&5), d.next());
    assert_eq!(Some(&2), d.next());
}

fn dump<T, U>(t: T)
where
    T: IntoIterator<Item = U>,
    U: Debug,
{
    for u in t {
        println!("{:?}", u);
    }
}

#[test]
fn test_dump() {
    let data = [1, 2, 3, 4];
    dump(data);
}

#[test]
fn test_generate_rand() {
    let rands = from_fn(|| Some(rand::random::<u8>()))
        .take(20)
        .collect::<Vec<u8>>();
    dump(rands);
}

fn simulating_fair_die(num: usize) -> usize {
    // uniformly-random [0, 1)
    let x: f64 = rand::random();
    let res = (num as f64 * x) as usize;
    res + 1
}

use std::collections::HashMap;

#[test]
fn test_rolling_die() {
    // for _ in 0..100 {
    //     rolling_die();
    // }

    count_random_and_dispaly(|| {
        let random_die = simulating_fair_die(6);
        random_die
    });
}

fn gcd(a: usize, b: usize) -> usize {
    if a % b == 0 {
        b
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

/// least common multiple with numbers
fn do_lcm(data: &[usize]) -> usize {
    if data.len() < 2 {
        panic!("expected at least 2 eles");
    }

    let mut _lcm = lcm(data[0], data[1]);
    if data.len() == 2 {
        return _lcm;
    }

    for i in 2..data.len() {
        _lcm = lcm(_lcm, data[i]);
    }

    return _lcm;
}

#[derive(Debug)]
struct Fraction {
    molecule: usize,
    denominator: usize,
}

impl Fraction {
    fn new(molecule: usize, denominator: usize) -> Self {
        Fraction {
            molecule,
            denominator,
        }
    }

    fn convert_to_lcm_format(&mut self, lcm: usize) {
        let molecule = (lcm / self.denominator) * self.molecule;
        self.molecule = molecule;
        self.denominator = lcm;
    }
}

fn gen_auxiliary_arr_of_values(
    probabilities: &mut [Fraction],
    weights: &[usize],
) -> (usize, Vec<usize>) {
    let denominators = probabilities
        .iter()
        .map(|f| f.denominator)
        .collect::<Vec<usize>>();
    let lcm = do_lcm(&denominators);
    let mut result = Vec::with_capacity(lcm);
    for ele in probabilities
        .iter_mut()
        .map(|f| {
            f.convert_to_lcm_format(lcm);
            f
        })
        .collect::<Vec<&mut Fraction>>()
        .iter()
        .enumerate()
    {
        let weight = weights[ele.0];
        for _ in 0..(ele.1.molecule) {
            result.push(weight);
        }
    }

    (lcm, result)
}

fn count_random_and_dispaly<F>(mut f: F)
where
    F: FnMut() -> usize,
{
    let mut counts = HashMap::new();
    for _ in 0..1000000 {
        let key = f();
        let count = if counts.contains_key(&key) {
            let val = *counts.get(&key).unwrap();
            let val = val + 1;
            val
        } else {
            1
        };
        counts.insert(key, count);
    }

    println!("{:?}", counts);
}

#[test]
fn test_lcm() {
    let lcm = do_lcm(&[1, 3, 5, 7, 2, 10]);
    println!("lcm is : {}", lcm);

    let mut probabilities = [
        Fraction::new(1, 36),
        Fraction::new(1, 36),
        Fraction::new(1, 9),
        Fraction::new(1, 2),
        Fraction::new(1, 3),
    ];
    let weights = [6, 12, 99, 0, 10];
    let original = gen_auxiliary_arr_of_values(&mut probabilities, &weights);
    count_random_and_dispaly(|| {
        let r = rand::random::<f64>();
        let idx = (r * original.0 as f64) as usize;
        (original.1)[idx]
    });
}

use std::boxed::Box;

use rand::prelude::ThreadRng;
struct AliasMethod<R: rand::Rng + ?Sized + Clone> {
    alias: Vec<usize>,
    probability: Vec<f64>,
    rng: Box<R>,
}

impl<R> AliasMethod<R>
where
    R: rand::Rng + ?Sized + Clone,
{
    fn new(probabilities: &mut [f64], rng: Box<R>) -> AliasMethod<R> {
        let mut p = vec![0.0; probabilities.len()];
        let mut a = vec![0_usize; probabilities.len()];
        let avg = 1.0_f64 / (probabilities.len() as f64);
        let mut large = Vec::new();
        let mut small = Vec::new();
        for (idx, &p) in probabilities.iter().enumerate() {
            if p >= avg {
                large.push(idx);
            } else {
                small.push(idx);
            }
        }

        loop {
            if !small.is_empty() && !large.is_empty() {
                let less = small.pop().unwrap();
                let more = large.pop().unwrap();
                println!("more is : {}, less is {}, p.len(): {}", more, less, p.len());
                p[less] = probabilities[less] * probabilities.len() as f64;
                a[less] = more;

                probabilities[more] = probabilities[more] + probabilities[less] - avg;

                if probabilities[more] >= 1.0_f64 / probabilities.len() as f64 {
                    large.push(more);
                } else {
                    small.push(more);
                }
            }
            break;
        }

        while !small.is_empty() {
            p[small.pop().unwrap()] = 1.0_f64;
        }

        while !large.is_empty() {
            p[large.pop().unwrap()] = 1.0_f64;
        }

        AliasMethod {
            alias: a,
            probability: p,
            rng: rng.clone(),
        }
    }

    fn next(&mut self) -> usize {
        let column = self.rng.gen_range(0..self.probability.len());
        let coin = self.rng.gen::<f64>() < self.probability[column];
        if coin {
            column
        } else {
            self.alias[column]
        }
    }
}

#[test]
fn test_alias_method() {
    let mut probabilitites = [0.09, 0.2, 0.3, 0.4, 0.01];
    let rng = rand::thread_rng();
    let rng = Box::new(rng);
    let mut alias_method = AliasMethod::<ThreadRng>::new(&mut probabilitites, rng);
    count_random_and_dispaly(|| alias_method.next())
}

#[test]
fn test_successors() {
    let sussor = std::iter::successors(Some((0, 1)), |f| {
        let b = f.0 + f.1;
        Some((f.1, b))
    });
    let fib = sussor.take(10).map(|f| f.1).collect::<Vec<i32>>();
    println!("fb: {:?}", fib);
}

#[test]
fn test_drain() {
    let mut outer = "Earth".to_string();
    let inner = String::from_iter(outer.drain(1..4));
    assert_eq!(outer, "Eh");
    assert_eq!(inner, "art");
    outer.push_str("MMM");
    assert_eq!(outer, "EhMMM");

    // Range
    println!(
        "{:?}",
        (1..10).step_by(2).into_iter().collect::<Vec<usize>>()
    );
    // RangeFrom
    println!("{:?}", (1..).into_iter().take(20).collect::<Vec<usize>>());
    // RangeInclusive
    println!(
        "{:?}",
        (1..=11).step_by(2).into_iter().collect::<Vec<usize>>()
    );
    let v = (1..)
        .step_by(3)
        .into_iter()
        .take(50)
        .collect::<Vec<usize>>();
    let mut c = v.chunks(3);
    let c_n = c.next().unwrap();
    println!("{:?}", v);
    println!("{:?}", c);
    println!("{:?}", c_n);
    let c_n = c.next().unwrap();
    println!("{:?}", c_n);
    println!("{:?}", v);
    println!("{:?}", c);
    let mut w = v.windows(5);
    let next = w.next().unwrap();
    println!("{:?}, next: {:?}", w, next);
    let next = w.next().unwrap();
    println!("{:?}, next: {:?}", w, next);

    let mut slice = (1..)
        .step_by(1)
        .into_iter()
        .take(20)
        .collect::<Vec<usize>>();
    let mut s = slice.split(|b| *b == 10);
    println!("{:?}", s.next());
    println!("{:?}", s.next());
    assert!(s.next().is_none());
    let mut s = slice.rsplit(|e| *e % 2 == 0);
    println!("{:?}", s.next());
    println!("{:?}", s.next());
    println!("{:?}", s.next());
    println!("{:?}", s.next());
    // assert!(s.next().is_none());
    println!("=============");
    let mut s = slice.splitn(3, |e| *e % 2 == 0);
    println!("{:?}", s.next());
    println!("{:?}", s.next());
    println!("{:?}", s.next());
    assert!(s.next().is_none());
    println!("=============");
    {
        let mut s = slice.splitn_mut(3, |e| *e % 2 == 0);
        let mut m = [2222, 333];
        {
            let mut s_next = s.next();
            s_next = Some(&mut m);
            println!("{:?}", s_next);
        }
        println!("{:?}", s);
    }

    println!("{:?}", slice);
}

#[test]
fn test_iter_adapter() {
    let text = "  PONIES  \n GIRAFFES \niguanas \nsquid".to_string();
    let v: Vec<&str> = text
        .lines()
        .map(str::trim)
        .filter(|s| *s != "iguanas")
        .collect();
    println!("{:?}", v);

    println!("{}", to_uppercase("hello World!!!"));
    println!(
        "{}",
        "hello world".to_string().drain(..5).collect::<String>()
    );

    let s = "good abc name good d m  ...".to_string();
    let new_s = s
        .split_whitespace()
        .skip_while(|&s| s == "good")
        // .skip(2)
        .map(str::to_string)
        .collect::<Vec<String>>();
    println!("new_s: {:?}", new_s);
}

fn to_uppercase(original: &str) -> String {
    original.chars().flat_map(char::to_uppercase).collect()
}

#[test]
fn test_by_ref() {
    let message = "To: jimb\r\n From: id\r\n\r\nOooooh, donuts!!\r\n";
    let mut lines = message.lines();
    println!("Headers:");
    for header in lines.by_ref().take_while(|l| !l.is_empty()) {
        println!("{}", header);
    }
    println!("\nBody:");
    for body in lines {
        println!("{}", body);
    }

    println!("message: {}", message);
}

#[test]
fn test_consumer() {
    let mut p = HashMap::new();
    p.insert("a", 123);
    p.insert("b", 99);
    p.insert("c", 1);
    p.insert("d", 999);

    println!("{:?}", p.iter().max_by_key(|(_, p)| *p).unwrap());
    println!("{:?}", p.iter().min_by_key(|(_, p)| *p).unwrap());

    let bytes = b"Xerxes";
    println!("{:?}", bytes);
    println!("{:?}", bytes.iter().rposition(|&c| c == b'e'));
    println!("{:?}", bytes.iter().rposition(|&c| c == b'X'));
    println!("{:?}", bytes.iter().position(|&c| c == b'e'));
    println!("{:?}", bytes.iter().position(|&c| c == b'X'));

    let a = [5, 6, 7, 8, 9, 10];
    // count
    assert_eq!(a.iter().fold(0, |n, _| n + 1), 6);
    // sum
    assert_eq!(a.iter().fold(0, |acc, e| acc + *e), 45);
    // product
    assert_eq!(a.iter().fold(1, |p, e| p * *e), 151200);
    // max
    assert_eq!(a.iter().copied().fold(i32::MIN, std::cmp::max), 10);
}

use std::error::Error;
use std::io::BufRead;
use std::str::FromStr;
#[test]
fn test_try_fold() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let sum = stdin
        .lock()
        .lines()
        .try_fold(0, |sum, line| -> Result<u64, Box<dyn Error>> {
            Ok(sum + u64::from_str(&line?.trim())?)
        })?;
    println!("sum is {}", sum);
    Ok(())
}

#[test]
fn test_foreach() {
    (1..10)
        .step_by(2)
        .zip(["a", "b", "c", "d"])
        .zip(["_goo", "_mm", "_ddd", "_uuu"])
        .rev()
        .map(|((a, b), c)| format!("{}{}{}", a, b, c))
        .for_each(|z| println!("got: {}", z));
}

struct I32Range {
    start: i32,
    end: i32,
}

impl Iterator for I32Range {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            return None;
        }

        let result = Some(self.start);
        self.start += 1;
        result
    }
}

#[test]
fn test_cus_iter() {
    let mut pi = 0.0;
    let mut numerator = 1.0;

    for k in (I32Range { start: 0, end: 14 }) {
        pi += numerator / (2 * k + 1) as f64;
        numerator /= -3.0;
    }

    pi *= f64::sqrt(12.0);
    println!("PI is : {}", &pi);
}

struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

impl<T> TreeNode<T> {
    fn new_node(element: T) -> TreeNode<T> {
        TreeNode {
            element,
            left: BinaryTree::Empty,
            right: BinaryTree::Empty,
        }
    }
}

enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

struct TreeIter<'a, T> {
    unvisited: Vec<&'a TreeNode<T>>,
}

impl<'a, T: 'a> TreeIter<'a, T> {
    fn push_left_edge(&mut self, mut tree: &'a BinaryTree<T>) {
        while let BinaryTree::NonEmpty(ref node) = *tree {
            self.unvisited.push(node);
            tree = &node.left;
        }
    }
}

impl<T: Ord + Clone> BinaryTree<T> {
    fn iter(&self) -> TreeIter<T> {
        let mut iter = TreeIter {
            unvisited: Vec::new(),
        };
        iter.push_left_edge(self);
        iter
    }

    fn new_node(new_node: TreeNode<T>) -> BinaryTree<T> {
        BinaryTree::NonEmpty(Box::new(new_node))
    }

    fn add(&mut self, element: T) {
        let new_node = BinaryTree::new_node(TreeNode::new_node(element.clone()));
        match *self {
            BinaryTree::NonEmpty(ref mut node) => {
                if element.clone() <= node.element {
                    node.left.add(element);
                } else {
                    node.right.add(element);
                }
            }
            BinaryTree::Empty => *self = new_node,
        }
    }
}

impl<'a, T: 'a + Ord + Clone> IntoIterator for &'a BinaryTree<T> {
    type Item = &'a T;
    type IntoIter = TreeIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T: 'a + Ord + Clone> Iterator for TreeIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let node = self.unvisited.pop()?;
        self.push_left_edge(&node.right);
        Some(&node.element)
    }
}

#[test]
fn test_b_tree() {
    let mut tree = BinaryTree::Empty;
    tree.add("java");
    tree.add("C++");
    tree.add("python");
    tree.add("go");
    tree.add("ruby");

    let mut v = Vec::new();
    for k in &tree {
        v.push((*k).to_string());
    }

    println!("{:?}", &v);
}

#[derive(Debug)]
enum State {
    NotFound(String),
    Ok,
}
#[test]
fn test_move() {
    let s = &State::NotFound("no such field".to_string());
    match *s {
        State::NotFound(ref _s) => {
            println!("hello: {}", &_s);
            println!("s is : {:?}", s);
        }
        _ => println!("others"),
    }

    println!("s is : {:?}", s);
}
