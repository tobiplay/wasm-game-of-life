use wasm_bindgen::prelude::*;

#[wasm_bindgen]
// This allows for each `Cell` to be represented as a single byte:
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// A `Cell` is one single square in our `Universe`.
///
/// It either is `Dead` (0) or `Alive` (1).
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    pub fn toggle(&mut self) {
        // Dereference the borrowd cell and match
        // against the two possible states:
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        };
    }
}
