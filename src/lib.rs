mod utils;

use wasm_bindgen::prelude::*;
// We need the std::fmt tools to print
// game state to the terminal:
use std::fmt;

// Crate to generate random numbers:
use rand::Rng;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Pull extern crate `web_sys` into scope:
extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hellooooo, wasm-game-of-life!");
}

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

#[wasm_bindgen]
/// The `Universe` stores a collection of `Cell` instances.
///
/// Each `Universe` is defined by a `width` and `height`,
/// which make up the grid and possible spots for all
/// `Cell` instances.
#[derive(PartialEq)]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    universe_option: UniverseOption,
}

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum UniverseOption {
    Random,
    TwoSeven,
    Dead,
}

// These functions won't be exposed to JavaScript as
// they're only meant to handle the internal state:
impl Universe {
    /// Returns the index of a `Cell` within an `Universe`.
    ///
    /// The calculation for the index is based on the design of linear memory.
    /// We multiply the `row` with the actual `width` of an `Universe` and add
    /// the `column` to the value.
    ///
    /// # Algorithm explanation
    ///
    /// Let's examine the return value for an example
    /// array. For this we'll assume an `Universe`
    /// with both a `width` and `height` of 3. We're looking
    /// for the `Cell` in the second row and second column,
    /// so the bottom right 'Cell', which contains a 8.
    /// The following matrix shows the columns and rows from
    /// 1 through 3.
    ///
    /// | | | | |
    /// |---|---|---|---|
    /// | | 1 | 2 | 3 |
    /// | 1 | 0 | 1 | 2 |
    /// | 2 | 3 | 4 | 5 |
    /// | 3 | 6 | 7 | 8 |
    ///
    /// We first multiply the `row` (2) and `width` of the
    /// `Universe` (3) and add the `column` (2). We get
    /// 8 as a result, which is exactly the index
    /// we're looking for (starting at 0).
    ///
    /// # Panics
    /// The method panics if at least one of `row` and
    /// `column` is negative.
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    /// Counts the living neighbours of a cell.
    ///
    /// We provide the `row` and `column` of the `Cell`
    /// for which we'd like to get the number of living
    /// neighbors returned.
    ///
    /// # Algorithm explanation
    ///
    /// The method uses deltas and modulo (`%`) to avoid special
    /// casing the edges of an `Universe` with `if` statements.
    /// When applying a delta of -1, we add `self.height`
    /// minus 1 and let the modulo do its thing, rather
    /// than attempting to subtract 1 directly. Both `row`
    /// and `column` can be 0, and if we attempted to subtract 1
    /// from them, there would be an unsigned integer underflow.
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        // Keep track of the number of living neighbors:
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                // When this is true, we're currently looking at the
                // center cell and not its neighbors, so we just continue.
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                // Calculate both the row and column of
                // the neighboring cell:
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                // Get the index of the neighboring cell:
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}

// These methods for `Universe` will be exposed to the
// JavaScript API:
#[wasm_bindgen]
impl Universe {
    /// Advances the time t one tick in time (= delta t).
    ///
    /// We achieve an advance in time by one tick by
    /// calculating the new state of each cell in the
    /// array (`Vec<Cell>`) and overwriting the
    /// previous state.
    pub fn tick(&mut self) {
        // Clone the current cells into a new vector:
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                // Get the index of the current cell:
                let idx = self.get_index(row, col);
                // Get the cell at the specific index:
                let cell = self.cells[idx];
                // Count the number of living neighbors:
                let live_neighbors = self.live_neighbor_count(row, col);

                // Log the amount of living cells and initial state to console output:
                // log!(
                //     "cell[{}, {}] is initially {:?} and has {} live neighbors",
                //     row,
                //     col,
                //     cell,
                //     live_neighbors
                // );

                // Determine the state of the cell in the next tick in time:
                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                // Log state change of each cell to console output:
                // log!("It becomes {:?}", next_cell);

                // Insert the `next_cell` into the array of
                // cells at the next tick in time:
                next[idx] = next_cell;
            }
        }

        // Overwrite the current array of cells with
        // those at the current time plus one tick:
        self.cells = next;
    }

    /// Creates and returns an instance of `Universe`.
    ///
    /// This specific instance has a `width` and `height`
    /// of 64. Takes an `UniverseOption` as an option, which
    /// allows for different starting universes. This state
    /// can be `TwoSeven`, where the index of each living starting
    /// cell was either divisible by 2 or 7, `Dead` or `Random`.
    pub fn new(universe_option: UniverseOption) -> Universe {
        // Enable logging for when our code panics.
        // This is achieved by invoking the set_panic_hook()
        // once somewhere in our code.
        utils::set_panic_hook();

        let universe_option = universe_option;
        let width = 64;
        let height = 64;

        /// Returns a vector of random `Cell` instances.
        ///
        /// The function takes in the `width` and `height` of
        /// the specifc universe as references.
        fn random_cells(width: &u32, height: &u32) -> Vec<Cell> {
            // Init a RNG thread:
            let mut rng = rand::thread_rng();

            let cells = (0..width * height)
                // And for each cell we map the following function
                // via a closure: if its index is divisable by 2
                // or by 7, it's a living cell. Otherwise it's dead.
                .map(|i| {
                    if rng.gen_range(0..=1) == 1 {
                        Cell::Alive
                    } else {
                        Cell::Dead
                    }
                })
                // Collect all cells into a vector:
                .collect();

            cells
        }

        /// Returns a vector of `Cell` instances in a specific pattern.
        ///
        /// The function takes in the `width` and `height` of
        /// the specifc universe as references. A living `Cell` is found
        /// at the start where the index of the cell is divisible by
        /// either 2 or 7.
        fn two_seven_cells(width: &u32, height: &u32) -> Vec<Cell> {
            let cells = (0..width * height)
                // And for each cell we map the following function
                // via a closure: if its index is divisable by 2
                // or by 7, it's a living cell. Otherwise it's dead.
                .map(|i| {
                    if i % 2 == 0 || i % 7 == 0 {
                        Cell::Alive
                    } else {
                        Cell::Dead
                    }
                })
                // Collect all cells into a vector:
                .collect();

            cells
        }

        /// Returns a vector of only dead `Cell` instances.
        ///
        /// The function takes in the `width` and `height` of
        /// the specifc universe as references.
        fn dead_cells(width: &u32, height: &u32) -> Vec<Cell> {
            let cells = (0..width * height).map(|i| Cell::Dead).collect();
            cells
        }

        // Create a range of cells with the correct
        // number of entries based on the width and height
        // of the Universe:
        let cells: Vec<Cell> = match universe_option {
            UniverseOption::Dead => dead_cells(&width, &height),
            UniverseOption::Random => random_cells(&width, &height),
            UniverseOption::TwoSeven => two_seven_cells(&width, &height),
        };

        // Return the universe:
        Universe {
            width,
            height,
            cells,
            universe_option,
        }
    }

    /// Returns the `Universe` as a `String`.
    ///
    /// This is possible, because we implemented the
    /// `Display` trait for `Universe`.
    pub fn render(&self) -> String {
        self.to_string()
    }

    // A couple more getter functions for our Universe,
    // which will be exposed to the JavaScript API.

    /// Returns the `width` of the `Universe`.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the `height` of the `Universe`.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Returns a raw pointer to the `cells` of
    /// the `Universe`.
    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
}

// Implementing the Display trait from Rust's standard library for Universe
// allows us to format the struct in a user-facing manner. We also gain
// access to the to_string method.
impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Slice the 2D array into lines based on the
        // width of a chunk which is also the width
        // of the Universe instance:
        for line in self.cells.as_slice().chunks(self.width as usize) {
            // For each cell in a line we then determine its state and write the
            // correct symbol:
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            // Jump to the next line after the end of the previous one:
            write!(f, "\n")?;
        }

        Ok(())
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Asserts that every property of the `Universe` struct was assigned.
    fn can_create_universe() {
        let universe = Universe::new(UniverseOption::TwoSeven);
        assert!(
            universe.height > 0
                && universe.width > 0
                && universe.cells.len() == universe.height as usize * universe.width as usize
        );
    }

    #[test]
    /// Calls every getter function that is going to be exposed to the JavaScript API.
    ///
    /// The JavaScript API and WASM are glued together via the `wasm_bindgen` glue,
    /// which relies on public getter functions to work with `Universe` and `Cell`
    /// structs outside of the Rust source.
    fn can_call_universe_getters() {
        // We test the getter functions on our basic TwoSeven universe.
        let universe = Universe::new(UniverseOption::TwoSeven);
        assert!(
            universe.height == universe.height()
                && universe.width == universe.width()
                && universe.cells.as_ptr() == universe.cells()
        );
    }

    #[test]
    /// Checks if the `Dead` option actually does not create a living cell.
    fn no_living_cells() {
        // To test the live_neighbor_count function we rely on
        // the Dead option for our universe. Here, no single cell
        // should be alive.
        let universe = Universe::new(UniverseOption::Dead);
        let count = universe.live_neighbor_count(1, 1);
        assert_eq!(count, 0);
    }

    #[test]
    /// Checks the the pattern created by the `TwoSeven` `UniverseOption`.
    fn two_seven_cells() {
        let universe = Universe::new(UniverseOption::TwoSeven);
        let count = universe.live_neighbor_count(1, 1);
        assert_eq!(count, 6);
    }

    #[test]
    /// Tests if the `tick` method does advance the board one tick in time.
    ///
    /// We use the `TwoSeven` option here, because it's easier to debug
    /// when the board is always the same.
    fn tick_does_advance() {
        let mut universe = Universe::new(UniverseOption::TwoSeven);
        // The cell at row 0, column 0 starts off as a living cell,
        // because its index is 0 and therefore divisible by 2.
        // The first cell should therefore be alive at the start:
        assert!(universe.cells[0] == Cell::Alive);

        // Advance one tick in time:
        universe.tick();

        // The cell should now be dead.
        assert!(universe.cells[0] == Cell::Dead)
    }
}
