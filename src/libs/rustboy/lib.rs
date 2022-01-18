mod chip8;
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

// example for using alert js module
#[wasm_bindgen]
pub fn greet(name: &str) {
    let message = format!("Hello {} using web-sys", name);
    alert(&message);
}

#[wasm_bindgen]
pub fn list_games() -> js_sys::Array {
    return chip8::games();
}

// boot rustboy
#[wasm_bindgen]
pub fn boot() {
    use web_sys::console;

    let games = chip8::games();

    console::log_1(&"Booting from WASM".into());
    console::log_1(&format!("{:?}", games).into());
    chip8::main();
}
