use ansi_term::Colour::Red;
use std::ops::Add;
/// Functional Language Features: Iterators and Closures.
use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    println!(
        "{}",
        Red.bold()
            .paint("================================")
            .to_string()
    );

    refactoring_functions::generate_workout(
        simulated_user_specified_value,
        simulated_random_number,
    );

    println!(
        "{}",
        Red.bold()
            .paint("================================")
            .to_string()
    );

    refactoring_closures::generate_workout(simulated_user_specified_value, simulated_random_number);
}

/// Closures: Anonymous Functions that Can Capture Their Environment.
/// 1. Creating an Abstraction of Behavior with Closures.
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today ,run for {} minutes",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}

pub(crate) mod refactoring_functions {
    use crate::simulated_expensive_calculation;

    pub fn generate_workout(intensity: u32, random_number: u32) {
        let expensive_result = simulated_expensive_calculation(intensity);

        if intensity < 25 {
            println!("Today, do {} pushups!", expensive_result);
            println!("Next, do {} situps!", expensive_result);
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!("Today ,run for {} minutes", expensive_result);
            }
        }
    }
}

pub(crate) mod refactoring_closures {
    use std::thread;
    use std::time::Duration;
    pub fn generate_workout(intensity: u32, random_number: u32) {
        let expensive_closure = |num: u32| {
            println!("calculating slowly...through closures...");
            thread::sleep(Duration::from_secs(2));
            num
        };

        if intensity < 25 {
            println!("Today, do {} pushups!", expensive_closure(intensity));
            println!("Next, do {} situps!", expensive_closure(intensity));
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!("Today ,run for {} minutes", expensive_closure(intensity));
            }
        }
    }
}

/// Storing Closures Using Generic Parameters and the Fn Traits
struct FnCache<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> FnCache<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> FnCache<T> {
        FnCache {
            // closure to execution
            calculation,
            // save the result of the expensive closure
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
            Some(v) => v,
        }
    }
}

mod struct_cache_refactoring {
    use crate::FnCache;
    use std::thread;
    use std::time::Duration;

    fn generate_workout(intensity: u32, random_number: u32) {
        let mut expensive_result = FnCache::new(|num: u32| {
            println!("calculating slowly through caching struct....");
            thread::sleep(Duration::from_secs(2));
            num
        });

        if intensity < 25 {
            println!("Today, do {} pushups!", expensive_result.value(intensity));
            println!("Next, do {} situps!", expensive_result.value(intensity));
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!(
                    "Today, run for {} minutes!",
                    expensive_result.value(intensity)
                );
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::FnCache;

        /// program bugs.
        #[test]
        fn call_with_different_values() {
            let mut c = FnCache::new(|a| a);
            let v1 = c.value(1);
            let v2 = c.value(2);
            // not equal here for 1 stored in FnCache(value)
            assert_eq!(v2, 2);
        }
    }
}

/// Fn FnOnce FnMut
/// move keyword
#[cfg(test)]
mod tests {

    #[test]
    fn main0() {
        let mut x = vec![1, 2, 3, 4];
        // move occurs here.
        // let equal_to_x = move |z| z == x;
        let push = |z| {
            x.push(z);
            println!("x is : {:?}", x);
            x
        };
        // println!("can't use x here: {:?}", x);
        let x = push(5);
        let y = vec![1, 2, 3, 4, 5];
        assert_eq!(x, y)
    }

    #[test]
    fn test_iter() {
        let v1 = vec![1, 2, 3, 4];
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
        assert_eq!(v2, vec![2, 3, 4, 5]);

        // filter etc...
    }

    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }

    #[test]
    fn filter_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: "sneaker".to_string(),
            },
            Shoe {
                size: 13,
                style: "sandal".to_string(),
            },
            Shoe {
                size: 10,
                style: "boot".to_string(),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);
        println!("{:?}", in_my_size);
    }

    struct Counter {
        count: usize,
    }

    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    #[test]
    fn test_iterator_counter() {
        let counter = Counter::new();
        for ct in counter {
            println!("ct is : {:?}", ct);
        }

        let sum: usize = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();

        assert_eq!(18, sum);
    }
}

/// must implement Add trait to execute '+' operator;
fn add<T>(i: T, j: T) -> T
where
    T: Add<Output = T>,
{
    i + j
}
