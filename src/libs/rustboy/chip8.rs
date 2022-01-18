use crate::utils; // i kind of hate this syntax

use wasm_bindgen::prelude::*;

// define our main mod function
// i might be doing something wrong but all the code samples for
// lib style exports seem to suggest allowing dead code  ¯\_(ツ)_/¯
#[wasm_bindgen]
pub fn main() {
  use web_sys::console;

  console::log_1(&"called `chip::main()`".into());
}

// couldnt work out how to 'export' a list of our games so...
// function to the rescue
#[wasm_bindgen]
pub fn games() -> js_sys::Array {
  return utils::to_array(&[
    "Invaders", "Brix", "Tetris", "Pong", "UFO", "IBM", "Missile", "Tank", "Maze",
  ]);
}
