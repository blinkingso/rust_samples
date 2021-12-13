//! String str...
#[test]
fn test_latin1() {
    // Ascii 0~127
    for i in 0..=127 {
        print!("{}", (i as u8) as char);
    }
    print!("\r\n");

    println!("{:?}", char_to_latin1('∂'));

    for i in 0..=0xff {
        print!("{}", latin1_to_char(i as u8));
    }
}

fn latin1_to_char(latin1: u8) -> char {
    latin1 as char
}

fn char_to_latin1(c: char) -> Option<u8> {
    // 0~255
    if c as u32 <= 0xff {
        Some(c as u8)
    } else {
        None
    }
}
use std::char;
use std::cmp::min;
use std::convert::From;
use std::ops::Range;

/// UTF-8
#[test]
fn test_utf_8() {
    // for i in 0x2f00..=0x2fff {
    //     let c = char::from_u32(i).unwrap_or(' ');
    //     // if c == '中' {
    //     //     println!("0x{:02x}", &i);
    //     //     break;
    //     // }
    //     print!("{}", c);
    // }

    println!("{:02x}", 'é' as u8);
    println!("{:02b}", 0xe9);
    let c = std::char::from_u32(0xfaa).unwrap_or(' ');
    println!("{}", c);
    assert_eq!("ערב טוב".chars().next(), Some('ע'));
    let c = 'd';
    assert!(c.is_alphabetic());
    println!("{:?}", 'z'.to_digit(15));
    // 0xa = 10;
    println!("{:?}", char::from_digit(10, 16));
    println!("{:?}", &mut '¶'.to_uppercase().next());
    for c in 'ß'.to_uppercase() {
        print!("{}", c);
    }
    println!();

    let c = char::from(66);
    println!("c is : {}", c);
}

struct M<'a> {
    data: &'a str,
}
fn mod_string(str: &mut String) -> M {
    // self.clone()
    let s = str.to_owned();
    println!("owned string is : {}", s);
    M { data: str }
}

/// String and str
#[test]
fn test_string_str() {
    let mut msg = String::from("hello world");
    {
        let new_msg = msg.drain(0..3);
        let new_msg = new_msg.as_str().to_string();
        println!("drain msg is: {}", new_msg);
    }
    println!("origin is : {}", msg);

    let spacey = "man hat tan";
    let spacess = spacey
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();
    println!("new spacey without space: {}", spacess);

    let mut s = String::new();
    s.push('c');
    s.push_str("_hello world");

    let c = &mut s;
    let m = mod_string(c);
    let mm = m.data.to_string();
    println!("mm is : {}", mm);
    println!("c is : {}", c);
    println!("s is : {}", s);

    let s = String::from("hello_my_books");
    println!("{}", s.as_str().is_char_boundary(5));
    let mut s = "hello".to_string();
    s.extend(" world".chars().into_iter());
    println!("new s is : {}", s);
    s.insert(0, 'n');
    println!("{}", s);
    s.insert_str(0, "to_");
    println!("{}", s);
    let mut s = String::new();
    use std::fmt::Write;
    let _ = writeln!(s, "hello, {}", "jack");
    println!("{}", s);
    s.truncate(3);
    println!("{}", s);
    let mut s = "个多字节的中文表示👌等字符".to_string();
    println!("s.len() : {}", s.len());
    s.replace_range(27..31, "信");
    println!("{}", s);
    let new_s = s.drain(..27).collect::<String>();
    println!("new_s : {}", new_s);
    let test_str = "数sdf据的👌🏻阿建瓯市地123方骄傲sdf";
    println!("{}", substring(test_str, 0..4));
    println!("{:?}", test_str.find('👌'));
    println!(
        "{}",
        "## Elements".trim_start_matches(|ch: char| ch == '#' || ch.is_whitespace())
    );
    let code = "\t      function noodle() { }";
    // &[char]
    println!("{}", code.trim_start_matches(&[' ', '\t'][..]));
    assert!("案件搜地".starts_with("案件"));
    assert!("1123a案件搜地发".starts_with(char::is_numeric));
    assert!("a12fsf".rfind(char::is_lowercase).is_some());
    println!("{}", "cabababababagge".replace("aba", "***"));
    println!("{}", "cabababababagge".replacen("aba", "***", 1));
}

fn substring(str: &str, range: Range<usize>) -> String {
    let len = str.chars().count();
    let start = min(len - 1, range.start);
    let end = min(len, range.end);

    let new_str = str
        .chars()
        .enumerate()
        .filter_map(move |(idx, ch)| {
            if idx >= start && idx < end {
                Some(ch)
            } else {
                None
            }
        })
        .collect::<String>();
    return new_str;
}

use unicode_normalization::UnicodeNormalization;
#[test]
fn test_unicode_normalization() {
    assert_eq!("test".nfc().collect::<String>(), "test");
}
