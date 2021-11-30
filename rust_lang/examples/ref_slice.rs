/// The Slice Type a special reference type.
fn main() {
    let mut s = String::from("hello world");
    let hello = &s[..5];
    let world = &s[6..];
    // s.clear();
    println!("hello is: {}, world is: {}", hello, world);
    let first_word = first_word(&s);
    let first_word = first_word.to_string();
    s.clear();
    println!("s cleared: {}", first_word);

    let mut s = "hello";
    s = "world";
    println!("s now is : {}", s);

    println!("{}", sub_string(8, "abc*(^中文咋还system"));
}

fn sub_string(idx: usize, source: &str) -> String {
    let mut s = String::new();
    let chars = source.chars();
    let mut index = 0usize;
    for c in chars {
        index += 1;
        if index < idx {
            s.push(c);
        }
    }
    s
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
