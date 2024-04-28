pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[cfg(target_arch = "wasm32")]
    mod wasm_tests {
        use super::{*, assert_eq};
        use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

        wasm_bindgen_test_configure!(run_in_browser);
    
        #[wasm_bindgen_test]
        fn test_add() {
            let result = add(2, 2);
            assert_eq!(result, 4);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    mod native_tests {
        use super::{*, assert_eq};

        #[test]
        fn test_add() {
            let result = add(2, 2);
            assert_eq!(result, 4);
        }
    }
}
