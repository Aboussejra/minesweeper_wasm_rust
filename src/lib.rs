mod minesweeper;
use minesweeper::Minesweeper;
use wasm_bindgen::prelude::*;

// Import the `window.alert` function from the Web.
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// Export a `greet` function from Rust to JavaScript, that alerts a
// hello message.
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen(js_name = getState)]
pub fn get_state() -> String {
    let minesweeper = Minesweeper::new(10,10,5);
    minesweeper.to_string()
}