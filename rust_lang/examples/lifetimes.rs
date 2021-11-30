/// lifetime in rust
fn main() {
    let longest = longest("abc", "a");
    println!("longest str is : {}", longest);

    let excerpt = &String::from("hello world")[..];
    let ie = ImportantExcerpt { part: excerpt };

    let level = ie.level();
    println!("level is : {}", level);

    let io_longest = ie.longest("small", "hello world");
    println!("ie longest is: {}", io_longest);

    let res = ie.announce_and_return_part("hello");
    println!("static lifetime: {}", res);
}

pub fn longest<'a, 'b: 'a>(str_a: &'a str, str_b: &'b str) -> &'a str {
    str_a
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        20
    }

    fn announce_and_return_part(&self, a1: &str) -> &str {
        println!("attention please: {}", a1);
        let s: &'static str = "I have a static lifetime?";
        // self.part
        s
    }

    fn longest(&self, a1: &'a str, a2: &'a str) -> &'a str {
        if a1.len() > a2.len() {
            a1
        } else {
            a2
        }
    }
}

#[cfg(test)]
mod tests {}
