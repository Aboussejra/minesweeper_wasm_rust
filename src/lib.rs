mod minesweeper;
use std::cell::RefCell;

use minesweeper::Minesweeper;
use wasm_bindgen::prelude::*;

thread_local! {
    static MINESWEEPER: std::cell::RefCell<Minesweeper>  = RefCell::new(Minesweeper::new(10,10,5));
}

#[wasm_bindgen(js_name = getState)]
pub fn get_state() -> String {
    MINESWEEPER.with(|ms| ms.borrow().to_string())
}

#[wasm_bindgen(js_name = openField)]
pub fn open_field(x: usize, y: usize) {
    MINESWEEPER.with(|ms| ms.borrow_mut().open((x, y)));
}

#[wasm_bindgen(js_name = toggleFlag)]
pub fn toggle_flag(x: usize, y: usize) {
    MINESWEEPER.with(|ms| ms.borrow_mut().toggle_flag((x, y)))
}

#[wasm_bindgen(js_name = resetGame)]
pub fn reset_game() {
    MINESWEEPER.with(|ms| ms.borrow_mut().reset())
}

#[wasm_bindgen(js_name = resetChangeMineNumber)]
pub fn reset_change_mine_number(mine_count: usize) {
    MINESWEEPER.with(|ms| ms.borrow_mut().change_bomb_number(mine_count))
}
