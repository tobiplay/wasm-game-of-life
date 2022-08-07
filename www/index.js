import * as wasm from "wasm-game-of-life";
// Specifically import the Universe struct:
import { Universe, Cell } from "wasm-game-of-life";
// Import the WebAssembly memory at the top of the file.
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

// Define some constants to represent cells:
const CELL_SIZE = 5; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

// Construct the universe, and get its width and height.
// The constructor function was built in Rust and compiled
// to WASM.
const universe = Universe.new();
const width = universe.width();
const height = universe.height();

// Grab the <canvas> element from the DOM:
const canvas = document.getElementById("game-of-life-canvas");
// Give the canvas room for all of our cells and a 1px border
// around each of them.
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext("2d");

// The JavaScript portion of our program runs in a
// requestAnimationFrame loop. On each iteration,
// it draws the current universe to the <canvas>,
// and then calls Universe::tick to advance one tick.
const renderLoop = () => {
    universe.tick();
    drawGrid();
    drawCells();

    requestAnimationFrame(renderLoop);
};

// To draw the grid between cells, we draw a set of equally-spaced
// horizontal lines, and a set of equally-spaced vertical lines.
// These lines criss-cross to form the grid.
const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // Vertical lines.
    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
};

const getIndex = (row, column) => {
    return row * width + column;
};

const drawCells = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);

            ctx.fillStyle = cells[idx] === Cell.Dead ? DEAD_COLOR : ALIVE_COLOR;

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
drawGrid();
drawCells();
requestAnimationFrame(renderLoop);

// This code is just meant as means to debug
// the JavaScript wasm_bindgen interface for now.
// wasm.greet();

var button = document.createElement("button");
button.innerHTML = "Do Something";

var body = document.getElementsByTagName("body")[0];
body.appendChild(button);

button.addEventListener("click", () => {});
