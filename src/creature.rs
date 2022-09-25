use wasm_bindgen::prelude::wasm_bindgen;

use crate::cell::CellContent;

#[wasm_bindgen]
pub struct Creature {
    brain: Brain,
    id: String
}

impl Creature {
    pub fn create() -> Self {
        Self {
            brain: Brain {},
            id: String::from("1234")
        }
    }
}

impl CellContent for Creature {
    fn draw(&self) {
        
    }
}

struct Brain {

}