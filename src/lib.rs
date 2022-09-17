// The mod keyword tells the compiler to look for the specified
// file in the module tree.
mod cell;
pub mod universe;
mod utils;

use wasm_bindgen::prelude::*;

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

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Asserts that every property of the `Universe` struct was assigned.
    fn can_create_universe() {
        let universe = universe::Universe::new(universe::UniverseOption::TwoSeven);
        assert!(
            universe.height() > 0
                && universe.width() > 0
                && universe.struct_cells().len()
                    == universe.height() as usize * universe.width() as usize
        );
    }

    #[test]
    /// Calls every getter function that is going to be exposed to the
    /// JavaScript API.
    ///
    /// The JavaScript API and WASM are glued together via the `wasm_bindgen`
    /// glue, which relies on public getter functions to work with `Universe`
    /// and `Cell` structs outside of the Rust source.
    fn can_call_universe_getters() {
        // We test the getter functions on our basic TwoSeven universe.
        let universe = universe::Universe::new(universe::UniverseOption::TwoSeven);
        assert!(
            universe.height() == universe.height()
                && universe.width() == universe.width()
                && universe.struct_cells().as_ptr() == universe.cells()
        );
    }

    #[test]
    /// Checks if the `Dead` option actually does not create a living cell.
    fn no_living_cells() {
        // To test the live_neighbor_count function we rely on
        // the Dead option for our universe. Here, no single cell
        // should be alive.
        let universe = universe::Universe::new(universe::UniverseOption::Dead);
        let count = universe.live_neighbor_count(1, 1);
        assert_eq!(count, 0);
    }

    #[test]
    /// Checks the the pattern created by the `TwoSeven` `UniverseOption`.
    fn two_seven_cells() {
        let universe = universe::Universe::new(universe::UniverseOption::TwoSeven);
        let count = universe.live_neighbor_count(1, 1);
        assert_eq!(count, 6);
    }

    #[test]
    fn can_toggle_cell() {
        let mut universe = universe::Universe::new(universe::UniverseOption::Dead);
        universe.toggle_cell(1, 1);
        assert_eq!(
            universe.struct_cells()[universe.get_index(1, 1)],
            cell::Cell::Alive
        );
    }

    #[test]
    fn can_toggle_glider() {
        let mut universe = universe::Universe::new(universe::UniverseOption::Dead);
        let center_of_universe = (universe.height() / 2, universe.width() / 2);

        // Toggle a glider in the center of the universe:
        universe.toggle_glider(center_of_universe.0, center_of_universe.1);

        // The glider should be in the center of the universe.
        let left = universe.get_index(center_of_universe.0, center_of_universe.1 - 1);
        let right = universe.get_index(center_of_universe.0, center_of_universe.1 + 1);
        let bottom = universe.get_index(center_of_universe.0 + 1, center_of_universe.1);
        let bottom_right = universe.get_index(center_of_universe.0 + 1, center_of_universe.1 + 1);
        let top_right = universe.get_index(center_of_universe.0 - 1, center_of_universe.1 + 1);

        assert_eq!(universe.struct_cells()[left], cell::Cell::Alive);
        assert_eq!(universe.struct_cells()[right], cell::Cell::Alive);
        assert_eq!(universe.struct_cells()[bottom], cell::Cell::Alive);
        assert_eq!(universe.struct_cells()[bottom_right], cell::Cell::Alive);
        assert_eq!(universe.struct_cells()[top_right], cell::Cell::Alive);

        // But the center cell should be dead:
        assert_eq!(
            universe.struct_cells()[universe.get_index(center_of_universe.0, center_of_universe.1)],
            cell::Cell::Dead
        );
    }
}
