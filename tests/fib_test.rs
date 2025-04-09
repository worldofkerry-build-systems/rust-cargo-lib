use fib::fibonacci;

#[test]
fn fib_test() {
    let result = fibonacci(10);
    assert_eq!(result, 55);
}