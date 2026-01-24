use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen] 属性 是把当前函数暴露给js使用

#[wasm_bindgen]
pub struct ArithmeticOperation;

// Internal error type for native testing
#[derive(Debug, PartialEq)]
pub enum ArithmeticError {
    Overflow,
    DivisionByZero,
    ModuloByZero,
}

impl ArithmeticOperation {
    /// Internal addition implementation for testing
    pub fn addition_internal(left: i32, right: i32) -> Result<i32, ArithmeticError> {
        left.checked_add(right)
            .ok_or(ArithmeticError::Overflow)
    }

    /// Internal subtraction implementation for testing
    pub fn subtraction_internal(left: i32, right: i32) -> Result<i32, ArithmeticError> {
        left.checked_sub(right)
            .ok_or(ArithmeticError::Overflow)
    }

    /// Internal division implementation for testing
    pub fn division_internal(left: i32, right: i32) -> Result<i32, ArithmeticError> {
        if right == 0 {
            return Err(ArithmeticError::DivisionByZero);
        }
        left.checked_div(right)
            .ok_or(ArithmeticError::Overflow)
    }

    /// Internal multiplication implementation for testing
    pub fn multiplication_internal(left: i32, right: i32) -> Result<i32, ArithmeticError> {
        left.checked_mul(right)
            .ok_or(ArithmeticError::Overflow)
    }

    /// Internal modulo implementation for testing
    pub fn modulo_internal(left: i32, right: i32) -> Result<i32, ArithmeticError> {
        if right == 0 {
            return Err(ArithmeticError::ModuloByZero);
        }
        left.checked_rem(right)
            .ok_or(ArithmeticError::Overflow)
    }

    /// Internal power implementation for testing
    pub fn power_internal(base: i32, exp: u32) -> Result<i32, ArithmeticError> {
        base.checked_pow(exp)
            .ok_or(ArithmeticError::Overflow)
    }
}

///
/// Arithmetic Operation (+,-,*,/)
/// 
/// This struct provides safe arithmetic operations with overflow checking
/// and proper error handling for edge cases like division by zero.
///
#[wasm_bindgen]
impl ArithmeticOperation {
    /// Addition with overflow checking.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust::ArithmeticOperation;
    /// let result = ArithmeticOperation::addition(1, 2); // 1 + 2 = 3
    /// assert_eq!(result, Ok(3));
    /// ```
    ///
    /// # Errors
    ///
    /// Returns error if the result overflows.
    #[wasm_bindgen]
    pub fn addition(left: i32, right: i32) -> Result<i32, JsValue> {
        Self::addition_internal(left, right)
            .map_err(|_| JsValue::from_str("Overflow in addition"))
    }

    /// Subtraction with overflow checking.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust::ArithmeticOperation;
    /// let result = ArithmeticOperation::subtraction(10, 9); // 10 - 9 = 1
    /// assert_eq!(result, Ok(1));
    /// ```
    ///
    /// # Errors
    ///
    /// Returns error if the result overflows.
    #[wasm_bindgen]
    pub fn subtraction(left: i32, right: i32) -> Result<i32, JsValue> {
        Self::subtraction_internal(left, right)
            .map_err(|_| JsValue::from_str("Overflow in subtraction"))
    }

    /// Division with zero checking.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust::ArithmeticOperation;
    /// let result = ArithmeticOperation::division(10, 5); // 10 / 5 = 2
    /// assert_eq!(result, Ok(2));
    /// ```
    ///
    /// # Errors
    ///
    /// Returns error if divisor is zero or overflow occurs.
    #[wasm_bindgen]
    pub fn division(left: i32, right: i32) -> Result<i32, JsValue> {
        Self::division_internal(left, right)
            .map_err(|e| match e {
                ArithmeticError::DivisionByZero => JsValue::from_str("Division by zero"),
                _ => JsValue::from_str("Overflow in division"),
            })
    }

    /// Multiplication with overflow checking.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust::ArithmeticOperation;
    /// let result = ArithmeticOperation::multiplication(2, 5); // 2 * 5 = 10
    /// assert_eq!(result, Ok(10));
    /// ```
    ///
    /// # Errors
    ///
    /// Returns error if the result overflows.
    #[wasm_bindgen]
    pub fn multiplication(left: i32, right: i32) -> Result<i32, JsValue> {
        Self::multiplication_internal(left, right)
            .map_err(|_| JsValue::from_str("Overflow in multiplication"))
    }

    /// Modulo operation with zero checking.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust::ArithmeticOperation;
    /// let result = ArithmeticOperation::modulo(10, 3); // 10 % 3 = 1
    /// assert_eq!(result, Ok(1));
    /// ```
    #[wasm_bindgen]
    pub fn modulo(left: i32, right: i32) -> Result<i32, JsValue> {
        Self::modulo_internal(left, right)
            .map_err(|e| match e {
                ArithmeticError::ModuloByZero => JsValue::from_str("Modulo by zero"),
                _ => JsValue::from_str("Overflow in modulo"),
            })
    }

    /// Power operation with overflow checking.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust::ArithmeticOperation;
    /// let result = ArithmeticOperation::power(2, 3); // 2 ^ 3 = 8
    /// assert_eq!(result, Ok(8));
    /// ```
    #[wasm_bindgen]
    pub fn power(base: i32, exp: u32) -> Result<i32, JsValue> {
        Self::power_internal(base, exp)
            .map_err(|_| JsValue::from_str("Overflow in power operation"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition_works() {
        let result = ArithmeticOperation::addition_internal(2, 2);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 4);
    }

    #[test]
    fn addition_overflow() {
        let result = ArithmeticOperation::addition_internal(i32::MAX, 1);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ArithmeticError::Overflow);
    }

    #[test]
    fn subtraction_works() {
        let result = ArithmeticOperation::subtraction_internal(10, 8);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2);
    }

    #[test]
    fn subtraction_overflow() {
        let result = ArithmeticOperation::subtraction_internal(i32::MIN, 1);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ArithmeticError::Overflow);
    }

    #[test]
    fn division_works() {
        let result = ArithmeticOperation::division_internal(10, 2);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 5);
    }

    #[test]
    fn division_by_zero() {
        let result = ArithmeticOperation::division_internal(10, 0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ArithmeticError::DivisionByZero);
    }

    #[test]
    fn multiplication_works() {
        let result = ArithmeticOperation::multiplication_internal(2, 5);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 10);
    }

    #[test]
    fn multiplication_overflow() {
        let result = ArithmeticOperation::multiplication_internal(i32::MAX, 2);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ArithmeticError::Overflow);
    }

    #[test]
    fn modulo_works() {
        let result = ArithmeticOperation::modulo_internal(10, 3);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }

    #[test]
    fn modulo_by_zero() {
        let result = ArithmeticOperation::modulo_internal(10, 0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ArithmeticError::ModuloByZero);
    }

    #[test]
    fn power_works() {
        let result = ArithmeticOperation::power_internal(2, 3);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 8);
    }

    #[test]
    fn power_overflow() {
        let result = ArithmeticOperation::power_internal(i32::MAX, 2);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ArithmeticError::Overflow);
    }
}
