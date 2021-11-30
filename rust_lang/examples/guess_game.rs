extern crate rand;
use rand::Rng;
use std::io;

fn main() {
    loop {
        let random: u32 = rand::thread_rng().gen_range(0..=100);
        println!("please guess: (0..100)");
        loop {
            let input_ = InputConfig::new();
            match input_ {
                Ok(conf) => {
                    let input = conf.input;
                    if random == input {
                        println!("you guess right: {}", random);
                        break;
                    } else if random > input {
                        println!("you guess smaller! guess again: ");
                    } else {
                        println!("you guess bigger! guess again: ");
                    }
                }
                Err(_) => {
                    println!("error input..., please re-input: ");
                }
            }
        }
    }
}

struct InputConfig {
    pub input: u32,
}

impl InputConfig {
    pub fn new() -> Result<Self, io::Error> {
        let mut number_str = String::new();
        let _ = io::stdin().read_line(&mut number_str)?;
        println!("input is : {}", number_str);
        let input = number_str.trim().parse::<u32>().map_err(|e| {
            println!("error is : {:?}", e);
            io::Error::from_raw_os_error(10024)
        })?;
        Ok(InputConfig { input })
    }
}
