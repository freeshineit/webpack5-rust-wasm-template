// 这个很重要
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen] 属性 是把当前函数暴露给js使用

/// Addition.
///
/// # Examples
///
/// ```
/// let x = addition(1, 2);
///
/// ```
#[wasm_bindgen]
pub fn addition(left: usize, right: usize) -> usize {
    left + right
}

/// Subtraction.
///
/// # Examples
///
/// ```
/// let x = subtraction(10, 9);
///
/// ```
#[wasm_bindgen]
pub fn subtraction(left: usize, right: usize) -> usize {
    left - right
}

/// Division.
///
/// # Examples
///
/// ```
/// let x = division(10, 5);
///
/// ```
#[wasm_bindgen]
pub fn division(left: usize, right: usize) -> usize {
    left / right
}

/// Multiplication.
///
/// # Examples
///
/// ```
/// let x = multiplication(2, 5);
///
/// ```
#[wasm_bindgen]
pub fn multiplication(left: usize, right: usize) -> usize {
    left * right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition_works() {
        let result = addition(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn subtraction_works() {
        let result = subtraction(10, 8);
        assert_eq!(result, 2);
    }

    #[test]
    fn division_works() {
        let result = division(10, 2);
        assert_eq!(result, 5);
    }

    #[test]
    fn multiplication_works() {
        let result = multiplication(2, 5);
        assert_eq!(result, 10);
    }
}
