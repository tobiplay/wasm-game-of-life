use wasm_bindgen::prelude::*;
// We need the std::fmt tools to print
// game state to the terminal:
use std::fmt;
// Crate to generate random numbers:
use rand::Rng;

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum UniverseOption {
    Random,
    TwoSeven,
    Dead,
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
    cells: Vec<super::cell::Cell>,
    universe_option: UniverseOption,
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
    pub fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    /// Returns the cells of the `Universe` struct.
    ///
    /// The cells of an `Universe` are hidden to the
    /// public API and have to be exposed with a separate
    /// function.
    pub fn struct_cells(&self) -> &Vec<super::cell::Cell> {
        &self.cells
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
    // fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
    //     // Keep track of the number of living neighbors:
    //     let mut count = 0;
    //     for delta_row in [self.height - 1, 0, 1].iter().cloned() {
    //         for delta_col in [self.width - 1, 0, 1].iter().cloned() {
    //             // When this is true, we're currently looking at the
    //             // center cell and not its neighbors, so we just continue.
    //             if delta_row == 0 && delta_col == 0 {
    //                 continue;
    //             }

    //             // Calculate both the row and column of
    //             // the neighboring cell:
    //             let neighbor_row = (row + delta_row) % self.height;
    //             let neighbor_col = (column + delta_col) % self.width;
    //             // Get the index of the neighboring cell:
    //             let idx = self.get_index(neighbor_row, neighbor_col);
    //             count += self.cells[idx] as u8;
    //         }
    //     }
    //     count
    // }

    /// Counts the living neighbours of a cell.
    ///
    /// We provide the `row` and `column` of the a cell
    /// for which we'd like to get the number of living
    /// neighbors (cells) returned.
    ///
    /// # Algorithm explanation
    ///
    /// First, we find the north, south, west and east cell to
    /// our current cell that we've defined via `row` and `column`.
    ///
    pub fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        // Determine the north, west, east and south cells:
        // If we're at the top row, we wrap to the last row by
        // subtracting 1 from the height of the universe. Note that
        // in Rust the index starts at 0, which is why we have to
        // substract 1. Otherwise we just return the row - 1.
        let north = if row == 0 { self.height - 1 } else { row - 1 };

        // If we're at the bottom row, we wrap to the first row by
        // returning a 0. Otherwise the row below is just the row + 1.
        let south = if row == self.height - 1 { 0 } else { row + 1 };

        // If we're at the leftmost column, we wrap to the rightmost
        // column by returning the width of the universe - 1. Otherwise
        // the column to the left is just the column - 1.
        let west = if column == 0 {
            self.width - 1
        } else {
            column - 1
        };

        // If we're at the rightmost column, we wrap to the leftmost
        // column by returning a 0. Otherwise the column to the right
        // is just the column + 1.
        let east = if column == self.width - 1 {
            0
        } else {
            column + 1
        };

        // We calculate the index of all neighboring cells and add their
        // value to our count variable. We can pass the direction as the
        // row and column to the get_index method, because they're
        // concrete identifiers for which columns and rows are adjacent to
        // our cell of interest.
        let nw = self.get_index(north, west);
        count += self.cells[nw] as u8;

        let n = self.get_index(north, column);
        count += self.cells[n] as u8;

        let ne = self.get_index(north, east);
        count += self.cells[ne] as u8;

        let w = self.get_index(row, west);
        count += self.cells[w] as u8;

        let e = self.get_index(row, east);
        count += self.cells[e] as u8;

        let sw = self.get_index(south, west);
        count += self.cells[sw] as u8;

        let s = self.get_index(south, column);
        count += self.cells[s] as u8;

        let se = self.get_index(south, east);
        count += self.cells[se] as u8;

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
        // Create a new Timer that tracks each tick of the universe.
        // Due to the nature of the tick method, a new timer is created
        // for each tick. When the timer is dropped, the time it took
        // to execute the tick method is printed to the console. The timer
        // is dropped at the end of the tick method, when it goes out of
        // scope.
        // let _timer = utils::Timer::new("Universe::tick");
        // Clone the current cells into a new vector:
        let mut next = self.cells.clone();
        // let _timer = utils::Timer::new("allocate new cells");

        {
            // let _timer = utils::Timer::new("new generation");
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
                        // Rule 1: Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.
                        (super::cell::Cell::Alive, x) if x < 2 => super::cell::Cell::Dead,
                        // Rule 2: Any live cell with two or three live neighbours lives on to the next generation.
                        (super::cell::Cell::Alive, 2) | (super::cell::Cell::Alive, 3) => {
                            super::cell::Cell::Alive
                        }
                        // Rule 3: Any live cell with more than three live
                        // neighbours dies, as if by overpopulation.
                        (super::cell::Cell::Alive, x) if x > 3 => super::cell::Cell::Dead,
                        // Rule 4: Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
                        (super::cell::Cell::Dead, 3) => super::cell::Cell::Alive,
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
        }
        // Overwrite the current array of cells with
        // those at the current time plus one tick:
        // let _timer = utils::Timer::new("free old cells");
        self.cells = next;
    }

    /// Creates and returns an instance of `Universe`.
    ///
    /// This specific instance has a `width` and `height`
    /// of 128. Takes an `UniverseOption` as an option, which
    /// allows for different starting universes. This state
    /// can be `TwoSeven`, where the index of each living starting
    /// cell was either divisible by 2 or 7, `Dead` or `Random`.
    pub fn new(universe_option: UniverseOption) -> Universe {
        // Enable logging for when our code panics.
        // This is achieved by invoking the set_panic_hook()
        // once somewhere in our code.
        super::utils::set_panic_hook();

        let universe_option = universe_option;
        let width = 256;
        let height = 256;

        /// Returns a vector of random `Cell` instances.
        ///
        /// The function takes in the `width` and `height` of
        /// the specifc universe as references.
        fn random_cells(width: &u32, height: &u32) -> Vec<super::cell::Cell> {
            // Init a RNG thread:
            let mut rng = rand::thread_rng();

            let cells = (0..width * height)
                // And for each cell we map the following function
                // via a closure: if its index is divisable by 2
                // or by 7, it's a living cell. Otherwise it's dead.
                .map(|i| {
                    if rng.gen_range(0..=1) == 1 {
                        super::cell::Cell::Alive
                    } else {
                        super::cell::Cell::Dead
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
        fn two_seven_cells(width: &u32, height: &u32) -> Vec<super::cell::Cell> {
            let cells = (0..width * height)
                // And for each cell we map the following function
                // via a closure: if its index is divisable by 2
                // or by 7, it's a living cell. Otherwise it's dead.
                .map(|i| {
                    if i % 2 == 0 || i % 7 == 0 {
                        super::cell::Cell::Alive
                    } else {
                        super::cell::Cell::Dead
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
        fn dead_cells(width: &u32, height: &u32) -> Vec<super::cell::Cell> {
            let cells = (0..width * height)
                .map(|i| super::cell::Cell::Dead)
                .collect();
            cells
        }

        // Create a range of cells with the correct
        // number of entries based on the width and height
        // of the Universe:
        let cells: Vec<super::cell::Cell> = match universe_option {
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

    /// Toggles the state of a cell.
    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
    }

    /// Inserts a glider pattern into the universe.
    pub fn toggle_glider(&mut self, row: u32, column: u32) {
        // Glider pattern, where x marks the clicked cell
        // and the other cells to be toggled are marked with a o:
        //     o
        // o x o
        //   o o
        // Grab the width and height of the universe as a tuple:
        let (width, height) = (self.width, self.height);

        // If the clicked cell is on the very right, left, top or bottom of
        // the universe, we can't insert the glider pattern, so return:
        if width - 1 == column || column == 0 || height - 1 == row || row == 0 {
            return;
        };

        let left = self.get_index(row, column - 1);
        let right = self.get_index(row, column + 1);
        let bottom = self.get_index(row + 1, column);
        let bottom_right = self.get_index(row + 1, column + 1);
        let top_right = self.get_index(row - 1, column + 1);

        let cells_to_toggle = [left, right, bottom, bottom_right, top_right];

        for cell in cells_to_toggle.iter() {
            self.cells[*cell].toggle();
        }
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
    pub fn cells(&self) -> *const super::cell::Cell {
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
                let symbol = if cell == super::cell::Cell::Dead {
                    '◻'
                } else {
                    '◼'
                };
                write!(f, "{}", symbol)?;
            }
            // Jump to the next line after the end of the previous one:
            write!(f, "\n")?;
        }

        Ok(())
    }
}
