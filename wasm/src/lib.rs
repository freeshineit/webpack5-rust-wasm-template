// 这个很重要
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen] 属性 是把当前函数暴露给js使用

#[wasm_bindgen]
struct ArithmeticOperation {}

///
/// Arithmetic Operation (+,-,*,/)
///
///
#[wasm_bindgen]
impl ArithmeticOperation {
    /// Addition.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = ArithmeticOperation::addition(1, 2);
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
    /// let x = ArithmeticOperation::subtraction(10, 9);
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
    /// let x = ArithmeticOperation::division(10, 5);
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
    /// let x = ArithmeticOperation::multiplication(2, 5);
    ///
    /// ```
    #[wasm_bindgen]
    pub fn multiplication(left: usize, right: usize) -> usize {
        left * right
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition_works() {
        let result = ArithmeticOperation::addition(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn subtraction_works() {
        let result = ArithmeticOperation::subtraction(10, 8);
        assert_eq!(result, 2);
    }

    #[test]
    fn division_works() {
        let result = ArithmeticOperation::division(10, 2);
        assert_eq!(result, 5);
    }

    #[test]
    fn multiplication_works() {
        let result = ArithmeticOperation::multiplication(2, 5);
        assert_eq!(result, 10);
    }
}
