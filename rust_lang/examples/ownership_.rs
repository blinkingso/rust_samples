/// 4. Understanding Ownership
/// stack and heap.
/// 1. ownership 3 rules:
/// 1.1. each value in Rust has a variable that's called its owner;
/// 1.2. there can only be one owner at a time;
/// 1.3. when the owner goes out of scope, the value will be dropped.
/// q: is there any fragmentation of memory?
/// 2. Variable Scope.
fn main() {}

// variable scope
fn variable_scope() {
    // s is not valid here, it's not yet declared

    let s = "hello"; // s is valid from this point forward.
} // this scope is now over, and s is no longer valid.

fn ownership_via_string() {
    let s = String::from("Hello");
    let mut s = String::from("hello");
    s.push_str(", world");

    println!("{}", s);

    // Q&A: Why can String be mutated but literals cannot?
    // The differences is how these two types deal with memory.
}

/// we known string literal contents at compile time, so the text is hardcoded directly into
/// the final executable. this is why string literals are fast and efficient.
/// But there properties only come from the string literal's immutability.
fn memory_and_allocation() {
    // s is valid from this point forward
    let s = String::from("hello");

    // do stuff with s;
} // this scope is now overs, and s is no longer valid.

/// multiple variables can interact with the same data in different ways in Rust.
fn ways_variables_and_data_interact_move() {
    // simple values with known, fixed size at compile time, so x, y these two 5 values are pushed
    // onto the stack.
    // bind the value 5 to `x`
    let x = 5;
    // assigning the integer value of variable `x` to `y`
    // make a copy of the value in `x` and bind it to `y`
    let y = x;

    // complex data type String.
    let s1 = String::from("hello");
    // s1 move here.
    let s2 = s1;

    // when we use s1 here, compiler error: for Rust considers s1 to no longer be valid and ,therefore,
    // Rust doesn't need to free anything when s1 goes out of scope.
    // To avoid Freeing memory twice to lead to memory corruption, which can potentially lead to
    // memory security vulnerabilities.
    // error here. s1 move here.
    // only s2 valid, when it goes out of scope, it alone will free the memory.
    // println!("{}, world!", s1);
}

/// Clone trait
/// deeply copy the heap data of the String.
fn ways_variables_and_data_interact_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

/// stack only data: Copy trait
/// Types such as integers that have a known size at compile time are stored
/// entirely on the stack, so copies of the actual values are quick to make.
/// That means there's no reason we would want to prevent `x` from being valid after we create
/// the variable `y`.
fn stack_only_data_copy() {
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
}

// #[derive(Copy, Clone)]
struct DeriveCopy {
    type_size: u32,
}

/// compile error, Cannot implement both Copy and Drop [E0184]
impl Drop for DeriveCopy {
    fn drop(&mut self) {
        println!("drop here");
    }
}

fn function_owner_move() {
    let s = String::from("hello");

    // s's value moves into the function...
    takes_ownership(s);
    // ...  and so is no longer valid here.
    // println!(s); // compile error here.

    let s = String::from("world");
    make_copy(s.as_str());

    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function, but i32 is Copy,
                   // so it's okay to still use x afterward
}

fn takes_ownership(message: String) {
    // message comes into scope.
    println!("message is {}", message);
} // Here, message goes out of scope, and `drop` is called. The backing memory is freed.

fn make_copy(message: &str) {
    println!("message is {}", message);
}

fn makes_copy(count: i32) {
    // count comes into scope
    println!("count is {}", count);
} // Here, count goes out of scope. Nothing special happens.
