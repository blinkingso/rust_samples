//! Iterators;

use std::iter::repeat;
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
