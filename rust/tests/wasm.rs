//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

use rust::ArithmeticOperation;

#[wasm_bindgen_test]
fn test_addition() {
    let result = ArithmeticOperation::addition_internal(2, 2);
    assert_eq!(result, Ok(4));
}

#[wasm_bindgen_test]
fn test_subtraction() {
    let result = ArithmeticOperation::subtraction_internal(10, 8);
    assert_eq!(result, Ok(2));
}

#[wasm_bindgen_test]
fn test_division() {
    let result = ArithmeticOperation::division_internal(10, 2);
    assert_eq!(result, Ok(5));
}

#[wasm_bindgen_test]
fn test_multiplication() {
    let result = ArithmeticOperation::multiplication_internal(2, 5);
    assert_eq!(result, Ok(10));
}

#[wasm_bindgen_test]
fn test_modulo() {
    let result = ArithmeticOperation::modulo_internal(10, 3);
    assert_eq!(result, Ok(1));
}

#[wasm_bindgen_test]
fn test_power() {
    let result = ArithmeticOperation::power_internal(2, 3);
    assert_eq!(result, Ok(8));
}
