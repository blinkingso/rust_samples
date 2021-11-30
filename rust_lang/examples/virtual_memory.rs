use std::boxed::Box;
static GLOBAL: i32 = 10000;
fn noop() -> *const i32 {
    let noop_local = 12345;
    &noop_local as *const i32
}
fn main() {
    let mut n_nonzero = 0;
    for i in 0..255 {
        let ptr = i as *const u8;
        println!("ptr is {:p}", ptr);
        // let byte_at_addr = unsafe { *ptr };

        // if byte_at_addr != 0 {
        // n_nonzero += 1;
        // }
    }

    println!("non-zero bytes in memory : {}", n_nonzero);

    let local_str = "a";
    let local_int = 123;
    let boxed_str = Box::new('b');
    let boxed_int = Box::new(1234);
    let fn_int = noop();

    println!("GLOBAL:           {:p}", &GLOBAL as *const i32);
    println!("local_str:        {:p}", local_str as *const str);
    println!("local_int:        {:p}", &local_int as *const i32);
    println!("boxed_int:        {:p}", Box::into_raw(boxed_int));
    println!("boxed_str:        {:p}", Box::into_raw(boxed_str.clone()));
    println!("fn_int:           {:p}", fn_int);
    let boxed_str_addr = unsafe { *Box::into_raw(boxed_str.clone()) };
    println!("boxed_str_addr is : {}", boxed_str_addr);

    // segmentation fault occures here.
    // let one_ptr_addr = unsafe { *(1 as *const u8) };
}
