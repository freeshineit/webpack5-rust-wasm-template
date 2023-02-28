// 这个很重要
use wasm_bindgen::prelude::*;

// #[wasm_bindgen] 属性 是把当前函数暴露给js使用
#[wasm_bindgen]
pub fn addition(left: usize, right: usize) -> usize {
    left + right
}

#[wasm_bindgen]
pub fn subtraction(left: usize, right: usize) -> usize {
    left - right
}

#[wasm_bindgen]
pub fn division(left: usize, right: usize) -> usize {
    left / right
}

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
