/// References and Borrowing.
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("length of '{}' is {}.", s1, len);
}

/// s here has a ptr to A String stack ref such as s1 in main
/// s (ptr) -> s1(ptr, len, capacity) -> String("hello") in heap.
fn calculate_length(s: &String) -> usize {
    s.len()
}

/// mutable ref
fn mutable_ref() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    // second mutable borrow occurs here
    let r2 = &mut s;
    // compile error here
    // println!("r1 is {}, r2 is {}", r1, r2);
    println!("r2 is {}", r2);
    {
        let r3 = &mut s;
        r3.push_str(", world");
    } // r3 goes out of scope here.
    println!("s is {}", s);

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
                 /*
                    let r3 = &mut s; // BIG PROBLEM
                    println!("{}, {}, and {}", r1, r2, r3);
                 */
    let r3 = &mut s; // NOW NO PROBLEM FOR r1,r2 un-using.
    println!("{}", r3);
}
