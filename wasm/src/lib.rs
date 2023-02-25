

// 这个很重要
use wasm_bindgen::prelude::*;

// #[wasm_bindgen] 属性 是把当前函数暴露给js使用
#[wasm_bindgen]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
