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
pub fn greet() {
    alert("Hellooooo, wasm-game-of-life!");
}

#[wasm_bindgen]
// This allows for each cell to be represented as a single byte:
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// A Cell is one single square in our universe.
///
/// It either is Dead (0) or Alive (1).
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
/// The `Universe` stores a collection of `Cell` instances.
///
/// Each `Universe` is defined by a `width` and `height`,
/// which make up the grid and possible spots for all
/// `Cell` instances.
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    /// Returns the index of a `Cell` within an `Universe`.
    ///
    /// The calculation for the index is based on the design of linear memory.
    /// We multiply the `row` with the actual `width` of an `Universe` and add
    /// the `column` to the value.
    ///
    /// # Explanation of algorithm
    ///
    /// Let's examine the return value for an example
    /// array. For this we'll assume an `Universe`
    /// with both a `width` and `height` of 3. We're looking
    /// for the `Cell` in the second row and second column,
    /// so the bottom right 'Cell', which contains a 8.
    /// ```
    /// [[0, 1, 2],
    ///  [3, 4, 5],
    ///  [6, 7, 8]]
    /// ```
    /// We first multiply the `row` (2) and `width` of the
    /// `Universe` (3) and add the `column` (2). We get
    /// 8 as a result, which is exactly the index
    /// we're looking for (starting at 0).
    ///
    /// # Panics
    /// The method panics if at least one of `row` and 
    /// `column` is negative.
    fn get_index(&self, row: u32, column: u32) -> usize {
        assert!(row > 0 && column > 0);
        (row * self.width + column) as usize
    }
}
