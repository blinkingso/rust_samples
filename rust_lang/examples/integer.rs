fn main() {}

#[test]
fn test_integer() {
    println!("{x}", x = 0b10011011_i32.count_ones());

    let mut i = 100_i8;
    let mut j = 0;
    loop {
        i = i.checked_add(20).unwrap_or(-128i8);
        println!("i is {}", i);
        if j == 10 {
            break;
        }
        j += 1;
    }

    assert_eq!(10_u32.checked_add(20), Some(30));
    assert_eq!(10_u8.checked_add(250), None);
    // i8 -128~127.
    assert_eq!((-128_i8).checked_div(-1), None);

    assert_eq!(100_u16.wrapping_mul(100), 10000);
    assert_eq!(127_i8.wrapping_add(1), 1);
}

#[test]
fn test_vector() {
    let mut primes = vec![1, 3, 5, 7, 9];
    primes.push(11);
    println!("mul is : {}", primes.iter().fold(1, |acct, ele| acct * ele));
    assert_eq!(primes.iter().product::<i32>(), 10395);
}
