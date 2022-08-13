import * as wasm from "wasm-game-of-life";
// Specifically import the Universe struct:
import { Universe, Cell, UniverseOption } from "wasm-game-of-life";
// Import the WebAssembly memory at the top of the file.
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

// Define some constants to represent cells:
const CELL_SIZE = 5; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

// We dynamically decide what type of starting universe
// to render based on this very option:
const universe_option = UniverseOption.TwoSeven;
// Construct the universe, and get its width and height.
// The constructor function was built in Rust and compiled
// to WASM.
const universe = Universe.new(universe_option);
const width = universe.width();
const height = universe.height();

// Grab the <canvas> element from the DOM:
const canvas = document.getElementById("game-of-life-canvas");
// Give the canvas room for all of our cells and a 1 px border
// around each of them. For each cell we add 1 px (border) and
// another 1 px to the whole canvas (outer border).
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

// Grab the canvas as a 2D context.
const ctx = canvas.getContext("2d");

// The JavaScript portion of our program runs in a
// requestAnimationFrame loop. On each iteration,
// it draws the current universe to the <canvas>,
// and then calls Universe::tick to advance one tick.
const renderLoop = () => {
    // Place a debugger checkpoint:
    debugger;
    // Tick once, after we had already called the
    // first tick before:
    universe.tick();
    // Draw both the grid and the cells:
    // drawGrid();
    drawCells();

    // console.log(universe.render());

    // Invoke the loop:
    requestAnimationFrame(renderLoop);
};

// To draw the grid between cells, we draw a set of equally-spaced
// horizontal lines, and a set of equally-spaced vertical lines.
// These lines criss-cross to form the grid.
/** Draws the grid on the `<canvas>` element. */
const drawGrid = () => {
    // Inside the <canvas> (grabbed via context):
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // Vertical lines.
    for (let i = 0; i <= width; i++) {
        // Move the pointer to the top position in the grid.
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        // Draw a line downwards from that position until
        // we reach the very bottom of the canvas.
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    // Horizontal lines. This works basically the same as the vertical
    // lines.
    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    // The stroke() methode actually draws the lines then.
    ctx.stroke();
};

/**
 * Returns the index of a `Cell`.
 * @param  {Number} row     The row of a `Universe`.
 * @param  {Number} column  The column of a `Universe`.
 * @return {String}         The index of a `Cell` at `row` and `column`.
 */
const getIndex = (row, column) => {
    return row * width + column;
};

/** Draws the cells composed of `Cell` instances on the `<canvas>`. */
const drawCells = () => {
    // Obtain a raw pointer to the memory, where the instances of `Cell`
    // are stored at.
    const cellsPtr = universe.cells();
    // Dump the cells into an array of unsigned 8-bit integers.
    // The array is supposed to be width * height in size.
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            // For each cell we obrain its index.
            const idx = getIndex(row, col);

            // Depending on the state of the cell we render
            // its associated rectangle in either one of those
            // two defined colors.
            ctx.fillStyle = cells[idx] === Cell.Dead ? DEAD_COLOR : ALIVE_COLOR;

            // We create a rectangle and shift 1 to the right for the
            // border on the left, and another one for each previous
            // cell due to its border too.
            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    ctx.stroke();
};

// To start the rendering process, all we have to do is
// make the initial call for the first iteration of the
// rendering loop. The reason we first draw the grid and
// the cells is so that the initial state of the universe
// is drawn before we make modifications by entering the
// renderLoop portion:
drawCells();
drawGrid();
requestAnimationFrame(renderLoop);

var button = document.createElement("button");
button.innerHTML = "Do Something";

var body = document.getElementsByTagName("body")[0];
body.appendChild(button);

button.addEventListener("click", () => {});
