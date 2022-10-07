/* tslint:disable */
/* eslint-disable */
/**
*/
export function greet(): void;
/**
*/
export enum UniverseOption {
  Random,
  TwoSeven,
  Dead,
}
/**
* A `Cell` is one single square in our `Universe`.
*
* It either is `Dead` (0) or `Alive` (1).
*/
export enum Cell {
  Dead,
  Alive,
}
/**
* The `Universe` stores a collection of `Cell` instances.
*
* Each `Universe` is defined by a `width` and `height`,
* which make up the grid and possible spots for all
* `Cell` instances.
*/
export class Universe {
  free(): void;
/**
* Advances the time t one tick in time (= delta t).
*
* We achieve an advance in time by one tick by
* calculating the new state of each cell in the
* array (`Vec<Cell>`) and overwriting the
* previous state.
*/
  tick(): void;
/**
* Creates and returns an instance of `Universe`.
*
* This specific instance has a `width` and `height`
* of 128. Takes an `UniverseOption` as an option, which
* allows for different starting universes. This state
* can be `TwoSeven`, where the index of each living starting
* cell was either divisible by 2 or 7, `Dead` or `Random`.
* @param {number} universe_option
* @param {number} width
* @param {number} height
* @returns {Universe}
*/
  static new(universe_option: number, width: number, height: number): Universe;
/**
* Returns the `Universe` as a `String`.
*
* This is possible, because we implemented the
* `Display` trait for `Universe`.
* @returns {string}
*/
  render(): string;
/**
* Toggles the state of a cell.
* @param {number} row
* @param {number} column
*/
  toggle_cell(row: number, column: number): void;
/**
* Inserts a glider pattern into the universe.
* @param {number} row
* @param {number} column
*/
  toggle_glider(row: number, column: number): void;
/**
* Returns the `width` of the `Universe`.
* @returns {number}
*/
  width(): number;
/**
* Returns the `height` of the `Universe`.
* @returns {number}
*/
  height(): number;
/**
* Returns a raw pointer to the `cells` of
* the `Universe`.
* @returns {number}
*/
  cells(): number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_universe_free: (a: number) => void;
  readonly universe_tick: (a: number) => void;
  readonly universe_new: (a: number, b: number, c: number) => number;
  readonly universe_render: (a: number, b: number) => void;
  readonly universe_toggle_cell: (a: number, b: number, c: number) => void;
  readonly universe_toggle_glider: (a: number, b: number, c: number) => void;
  readonly universe_width: (a: number) => number;
  readonly universe_height: (a: number) => number;
  readonly universe_cells: (a: number) => number;
  readonly greet: () => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
