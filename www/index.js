// Import the fps module.
import { fps } from "./utils/fps.js";
// Specifically import the Universe struct:
import { Universe, Cell, UniverseOption } from "wasm-game-of-life";
// Import the WebAssembly memory at the top of the file.
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

// Define some constants to represent cells:
const CELL_SIZE = 5; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

// Init a new universe with the default option (Dead):
let universe = Universe.new(UniverseOption.Dead);

// We dynamically decide what type of starting universe
// to render based on the universeSelector, which can be used
// by the user on the front-end. We therefore grab the value
// of the select-universe element:
const universeSelector = document.getElementById("select-universe");

// And listen to any changes on it:
universeSelector.addEventListener("change", (e) => {
  let selectedUniverseOption = universeSelector.value;

  let universeOption = null;

  if (selectedUniverseOption === "Random") {
    universeOption = UniverseOption.Random;
  } else if (selectedUniverseOption === "TwoSeven") {
    universeOption = UniverseOption.TwoSeven;
  } else {
    universeOption = UniverseOption.Dead;
  }

  universe = Universe.new(universeOption);
  drawCells();
});

// Now that an universe exists, we can grab the height and width:
let width = universe.width();
let height = universe.height();

// Grab the <canvas> element from the DOM:
const canvas = document.getElementById("game-of-life-canvas");
// Give the canvas room for all of our cells and a 1 px border
// around each of them. For each cell we add 1 px (border) and
// another 1 px to the whole canvas (outer border).
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

// Grab the canvas as a 2D context.
const ctx = canvas.getContext("2d");

// Compute the click action on the canvas element.
canvas.addEventListener("click", (e) => {
  // We listen to the click event within the rectangle that
  // makes up the canvas element:F
  const boundingRect = canvas.getBoundingClientRect();

  // We calculate the relative scale in X and Y direction:
  const scaleX = canvas.width / boundingRect.width;
  const scaleY = canvas.height / boundingRect.height;

  // We calculate the position of the click within the rectangle:
  const canvasLeft = (e.clientX - boundingRect.left) * scaleX;
  const canvasTop = (e.clientY - boundingRect.top) * scaleY;

  // Transform coursor (click) position into a row and col.
  // We divide the position within the canvas by the size of each cell.
  // This yields the amount of cells. We have to add 1 to each cell size,
  // due to the border. We then take the floor of that, because we want to
  // stay within a certain cell. We take the minimum of that result
  // and 1 px less than the height of the `Universe` in cases where we might
  // land exactly on the border.
  const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
  const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

  if (e.shiftKey) {
    universe.toggle_glider(row, col);
  } else if (e.altKey) {
  } else {
    // Toggle the cell state via the WASM function when no
    // modifier keys are pressed.
    universe.toggle_cell(row, col);
  }

  // Redraw the cells after the click:
  drawCells();
});

// Grab the reset button.
const resetButton = document.getElementById("reset");
// Set the text of the button to "Reset":
resetButton.textContent = "Reset";
// Listen to the click event on the reset button.
resetButton.addEventListener("click", (e) => {
  // Cancel the animation:
  cancelAnimationFrame(animationId);
  // Reset the animationId:
  animationId = null;

  // Set the value of the universeSelector to "Empty":
  universeSelector.value = "Empty";
  // Reset the play button:
  playPauseButton.textContent = "Play";

  // Reset the universe:
  universe = Universe.new(UniverseOption.Dead);

  // Redraw the cells:
  drawCells();
});

// Grab the button.
const playPauseButton = document.getElementById("play-pause");

const play = () => {
  playPauseButton.textContent = "Pause";
  // Restart the animation by requesting the renderLoop:
  renderLoop();
};

const pause = () => {
  playPauseButton.textContent = "Play";
  // Cancel the animation via the animationId:
  cancelAnimationFrame(animationId);
  animationId = null;
};

playPauseButton.addEventListener("click", (e) => {
  isPaused() ? play() : pause();
});

// We store an animationId, which will be used
// as an identifier to stop and continue the loop.
let animationId = null;

// We can tell if the renderLoop is paused
// by checking the animationId.
const isPaused = () => {
  return animationId === null;
};

// Because we want the user to be able to decide how
// many ticks make up 1 frame on the screen, we have
// to keep track of his decision, which is input via
// the slider on the front-end. So we first grab
// the slider from the DOM:
const slider = document.getElementById("ticks-per-frame");
// We then store the selected value from the slider in a variable:
let ticksPerFrame = slider.value;

slider.addEventListener("change", (e) => {
  ticksPerFrame = slider.value;
});

// The JavaScript portion of our program runs in a
// requestAnimationFrame loop. On each iteration,
// it draws the current universe to the <canvas>,
// and then calls Universe::tick to advance one tick.
const renderLoop = () => {
  fps.render();
  // Place a debugger checkpoint:
  // debugger;

  // We'll tick the universe as many times as the user
  // has selected via the slider:
  for (let i = 0; i < ticksPerFrame; i++) {
    universe.tick();
  }

  // If we wante to redraw the grid, we would
  // have to to uncomment this line:
  // drawGrid();

  // We only draw the cells every ticksPerFrame iterations.
  drawCells();

  // console.log(universe.render());
  // Invoke the loop and store the animationId.
  animationId = requestAnimationFrame(renderLoop);
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

  // Alive cells. Because the setter for the `fillStyle` property
  // turns out to be so expensive, we only set it once for all
  // alive cells.
  ctx.fillStyle = ALIVE_COLOR;
  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      // For each cell we obrain its index.
      const idx = getIndex(row, col);
      if (cells[idx] !== Cell.Alive) {
        continue;
      }

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

  // Dead cells. We repeat the same for dead cells. By only setting
  // the `fillStyle` property once, we can save a lot of time for
  // the tick method.
  ctx.fillStyle = DEAD_COLOR;
  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);
      if (cells[idx] !== Cell.Dead) {
        continue;
      }

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
fps.render();
pause();

// var button = document.createElement("button");
// button.innerHTML = "Do Something";

// var body = document.getElementsByTagName("body")[0];
// body.appendChild(button);

// button.addEventListener("click", () => {});
