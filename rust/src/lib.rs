mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn justgreet() {
    alert("Hello, rust!");
}

#[wasm_bindgen]
pub fn greet(custom_message: &str) {
    alert(custom_message);
}