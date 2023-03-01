// wasm 是 `Cargo.toml`下 `package`的name
use wasm::ArithmeticOperation;

#[test]
fn test_addition() {
    let result = ArithmeticOperation::addition(2, 2);
    assert_eq!(result, 4);
}

#[test]
fn test_subtraction() {
    let result = ArithmeticOperation::subtraction(10, 8);
    assert_eq!(result, 2);
}

#[test]
fn test_division() {
    let result = ArithmeticOperation::division(10, 2);
    assert_eq!(result, 5);
}

#[test]
fn test_multiplication() {
    let result = ArithmeticOperation::multiplication(2, 5);
    assert_eq!(result, 10);
}
