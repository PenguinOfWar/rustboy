mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let message = format!("Hello {} using web-sys", name);
    alert(&message);
}

// custom loader for rustboy
#[wasm_bindgen]
pub fn boot() {
    use web_sys::console;

    console::log_1(&"Booting from WASM".into());
}
