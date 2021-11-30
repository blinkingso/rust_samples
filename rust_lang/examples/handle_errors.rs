/// error handling
fn main() {}

#[cfg(test)]
mod tests {
    use rand::Rng;
    use std::cmp::Ordering;

    #[test]
    fn test_panic_abort() {
        let v = vec![1, 2, 3, 4];
        // let i = &v[100];
        // println!("i is : {}", i);

        let num = rand::thread_rng().gen_range(0..=100);
        let mut times = 0u32;
        loop {
            let rand: i32 = rand::thread_rng().gen_range(0..100);
            let guess = Guess::new(rand);
            match guess.value().cmp(&num) {
                Ordering::Less => {
                    println!("guess smaller");
                    times += 1;
                    continue;
                }
                Ordering::Equal => {
                    println!("guess right: {}, guess times: {}", rand, times);
                    break;
                }
                Ordering::Greater => {
                    times += 1;
                    println!("guess bigger");
                    continue;
                }
            }
        }
    }

    struct Guess {
        value: i32,
    }
    impl Guess {
        pub(crate) fn new(guess: i32) -> Guess {
            println!("your guess is : {}", guess);
            if guess < 1 || guess > 100 {
                panic!("number must between 1 and 100");
            }

            Guess { value: guess }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }
}
