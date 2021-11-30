//! Memory Pointers
static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];
static D: i32 = 101;
use std::boxed::Box;
use std::cell::RefCell;
use std::mem::size_of;
fn main() {
    let a = 42;
    let b = &B;
    let c = Box::new(C);
    // {:p} syntax asks Rust to format the variable as a pointer and
    // prints the memory address that the value points to.
    println!("a: {}, b: {:p}, c: {:p}, C: {:p}, d: {:p}", a, b, c, &C, &D);

    println!("a (an unsigned integer): ");
    println!("location :    {:p}", &a);
    println!("size :        {:?} bytes", size_of::<usize>());
    println!("points to :   {:?}", a);

    println!("b (a reference to B): ");
    println!("location :    {:p}", &b);
    println!("size :        {:?} bytes", size_of::<&[u8; 10]>());
    println!("points to :   {:p}", b);

    println!(r#"c (a "box" for C): "#);
    println!("location :    {:p}", &c);
    println!("size :        {:?} bytes", size_of::<Box<[u8]>>());
    println!("points to :   {:p}", c);

    println!("B (an array of 10 bytes): ");
    println!("location :    {:p}", &B);
    println!("size :        {:?} bytes", size_of::<[u8; 10]>());
    println!("points to :   {:?}", B);

    println!("C (an array of 11 bytes): ");
    println!("location :    {:p}", &c);
    println!("size :        {:?} bytes", size_of::<[u8; 11]>());
    println!("points to :   {:?}", C);

    println!(
        "size of type are {}, {}",
        size_of::<&[u8; 10]>(),
        size_of::<&[u8; 11]>()
    );
}

#[test]
fn test_strings() {
    use std::borrow::Cow;
    use std::ffi::CStr;
    use std::os::raw::c_char;

    let a = 42;

    let c: Cow<str>;

    unsafe {
        let b_ptr = &B as *const u8 as *mut u8;
        let b = String::from_raw_parts(b_ptr, 10, 10);
        let c_ptr = &C as *const u8 as *const c_char;
        c = CStr::from_ptr(c_ptr).to_string_lossy();
    }
    // println!("a: {}, b: {}, c: {}", a, b, c);
    // println!("a: {}, c: {}", a, c);
}

#[test]
fn test_raw_pointers() {
    let a: i64 = 42;
    let a_ptr = &a as *const i64;
    println!("a: {}, ({:p})", a, a_ptr);
}

#[test]
fn test_address() {
    use std::mem::transmute;
    let a: i64 = 42;
    let a_ptr = &a as *const i64;
    let a_addr: usize = unsafe { transmute(a_ptr) };
    println!("a: {} ({:p}...=0x{:x})", a, a_ptr, a_addr + 7);
    let ptr = 56 as *const Vec<String>;
    unsafe {
        let new_addt = ptr.offset(4);
        println!("{:p} -> {:p}", ptr, new_addt);

        let mut new_vec = Vec::from_raw_parts(new_addt as *mut String, 0, 0);
        new_vec.push("hello".to_string());
        println!("new_vec is: {:?}", new_vec);
        let sa = String::from("hello world");
        let a_ptr = &sa as *const String as *mut String;
        drop(sa);
        a_ptr.offset(5);
        println!("a_ptr is {:p}, but sa has been dropped.", a_ptr);
        // let new_str = String::from_raw_parts(a_ptr as *mut u8, 5, 8);
        // println!("new_str is {}", new_str);
        drop(a_ptr);
    }
}

#[test]
fn test_smart_pointers() {
    use std::cell::Cell;
    let a = 42;
    let mut mut_a = Cell::new(a);
    *mut_a.get_mut() += 10;
    println!("new a is : {}", mut_a.get());
    println!("a is : {}", a);
    let a = String::from("hello");
    let mut mut_a = Cell::new(a);
    mut_a.get_mut().push_str(" world");
    println!("new str is : {:?}", mut_a.get_mut());

    let a = String::from("hl");
    let s = RefCell::from(a);
    s.borrow_mut().push_str("sss");
    println!("new str is : {}", s.borrow());

    let sti = IntoOpt {
        page: "page into".to_string(),
        no: 102,
    };
    let sti: String = sti.into();
    println!("{}", sti);

    println!("{}", Box::new(10));
    let c = 40 + *Box::new(20);
    println!("c = {}", c);

    let mut a = Box::new(120);
    drop(a);
    a = Box::new(220);
    println!("new a is :{}", a);
}

pub struct IntoOpt {
    page: String,
    no: u32,
}

impl Into<String> for IntoOpt {
    fn into(self) -> String {
        let page = self.page;
        let no = self.no;
        return String::from(format!("{}->>{}", page, no));
    }
}
