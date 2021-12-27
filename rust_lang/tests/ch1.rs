#[test]
fn test_variables() {
    let mut x;
    x = 42;
    let y = &x;
    // x = 43;
    assert_eq!(*y, 42);
}
