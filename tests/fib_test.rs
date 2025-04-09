use crate::fib::fib;

#[test]
fn fib_test() {
    let result = fib(10);
    assert_eq!(result == 55);
}