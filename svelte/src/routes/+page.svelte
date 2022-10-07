<script lang="ts">
  import { onMount } from 'svelte';
  import init, { greet } from 'wasm-game-of-life';
  import { Universe, Cell, UniverseOption } from 'wasm-game-of-life';
  import Fps from '../components/fpsCounter.svelte';

  let fpsComponent: any;

  let canvas: any;
  let ticksPerFrame: number = 1;
  let universe: any;
  let ctx: any;
  let height: number;
  let width: number;
  let frame;
  let wasm;
  let memory: any;
  let gridSize: number = 64;

  onMount(async () => {
    // We need to init the WASM module once before we can use it.
    wasm = await init();
    memory = wasm.memory;

    ctx = canvas.getContext('2d');

    universe = Universe.new(UniverseOption.Dead, gridSize, gridSize);

    // Now that an universe exists, we can grab the height and width:
    width = universe.width();
    height = universe.height();

    // Give the canvas room for all of our cells and a 1 px border
    // around each of them. For each cell we add 1 px (border) and
    // another 1 px to the whole canvas (outer border).
    canvas.height = (CELL_SIZE + 1) * height + 1;
    canvas.width = (CELL_SIZE + 1) * width + 1;

    drawGrid();
    drawCells();
    // render();
    pause();
  });

  // Define some constants to represent cells:
  const CELL_SIZE = 5; // Unit is px.
  const GRID_COLOR = '#CCCCCC';
  // const GRID_COLOR = '#FFFFFF';
  const DEAD_COLOR = '#FFFFFF';
  const ALIVE_COLOR = '#00FF00';

  // const DYING_1_COLOR = '#0a2d27';
  // const DYING_2_COLOR = '#13594e';
  // const DYING_3_COLOR = '#1d8676';
  // const DYING_4_COLOR = '#26b29d';
  // const DYING_5_COLOR = '#30dfc4';
  // const DYING_6_COLOR = '#59e5d0';
  // const DYING_7_COLOR = '#83ecdc';
  // const DYING_8_COLOR = '#acf2e7';
  // const DYING_9_COLOR = '#d6f9f3';

  let universeOptions = [
    { id: 0, text: 'Empty' },
    { id: 1, text: 'Random' },
    { id: 2, text: 'TwoSeven' }
  ];

  let selected: any;

  /**
   * Returns the index of a `Cell`.
   * @param  {Number} row     The row of a `Universe`.
   * @param  {Number} column  The column of a `Universe`.
   * @return {String}         The index of a `Cell` at `row` and `column`.
   */
  const getIndex = (row: number, column: number) => {
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
        ctx.fillRect(col * (CELL_SIZE + 1) + 1, row * (CELL_SIZE + 1) + 1, CELL_SIZE, CELL_SIZE);
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

        ctx.fillRect(col * (CELL_SIZE + 1) + 1, row * (CELL_SIZE + 1) + 1, CELL_SIZE, CELL_SIZE);
      }
    }

    ctx.stroke();
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

  function handleUniverseOptionChange() {
    if (selected.id === universeOptions[0].id) {
      universe = Universe.new(UniverseOption.Dead, gridSize, gridSize);
    } else if (selected.id === universeOptions[1].id) {
      universe = Universe.new(UniverseOption.Random, gridSize, gridSize);
    } else if (selected.id === universeOptions[2].id) {
      universe = Universe.new(UniverseOption.TwoSeven, gridSize, gridSize);
    }
    drawCells();
  }

  function handleCanvasClick(e: any) {
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
  }

  let playPauseState = 'Pause';

  const play = () => {
    playPauseState = 'Pause';
    // Restart the animation by requesting the renderLoop:
    renderLoop();
  };

  const pause = () => {
    playPauseState = 'Play';
    // Cancel the animation via the animationId:
    cancelAnimationFrame(animationId);
    animationId = null;
  };

  const handlePlayPauseClick = () => {
    isPaused() ? play() : pause();
  };

  const handleResetClick = () => {
    pause();
    selected = universeOptions[0];
    universe = Universe.new(UniverseOption.Dead, gridSize, gridSize);
    drawCells();
  };

  // We store an animationId, which will be used
  // as an identifier to stop and continue the loop.
  let animationId: any = null;

  // We can tell if the renderLoop is paused
  // by checking the animationId.
  const isPaused = () => {
    return animationId === null;
  };

  const handleGridSizeChange = () => {
    handleResetClick();
    ctx = canvas.getContext('2d');
    universe = Universe.new(UniverseOption.Dead, gridSize, gridSize);
    width = universe.width();
    height = universe.height();
    canvas.height = (CELL_SIZE + 1) * height + 1;
    canvas.width = (CELL_SIZE + 1) * width + 1;
    drawGrid();
  };

  // The JavaScript portion of our program runs in a
  // requestAnimationFrame loop. On each iteration,
  // it draws the current universe to the <canvas>,
  // and then calls Universe::tick to advance one tick.
  const renderLoop = () => {
    fpsComponent.renderFpsComponent();

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
</script>

<h1>Conway's Game of Life</h1>
<p>
  Visit the <a style="margin-inline: .25em;" href="https://github.com/tobiplay/wasm-game-of-life"
    >repository on GitHub</a
  > to read about the project.
</p>
<form>
  <label for="select-universe">Choose a starting universe:</label>
  <select
    name="select-universe"
    id="select-universe"
    bind:value={selected}
    on:change={handleUniverseOptionChange}
  >
    {#each universeOptions as option}
      <option value={option}>
        {option.text}
      </option>
    {/each}
  </select>
</form>
<div>
  <button id="play-pause" on:click={handlePlayPauseClick}>{playPauseState}</button>
  <button id="reset" on:click={handleResetClick}>Reset</button>
</div>

<Fps bind:this={fpsComponent} />
<canvas bind:this={canvas} on:click={handleCanvasClick} />
<form>
  <label for="ticks-per-frame">Ticks per frame = {ticksPerFrame}</label>
  <br />
  <input bind:value={ticksPerFrame} type="range" id="ticks-per-frame" min="1" max="10" />
</form>
<form>
  <label for="ticks-per-frame">Size of grid = {gridSize}</label>
  <br />
  <input
    bind:value={gridSize}
    type="range"
    id="ticks-per-frame"
    min="16"
    max="224"
    step="16"
    on:change={handleGridSizeChange}
  />
</form>

<style>
  p,
  form,
  button,
  h1,
  div {
    justify-content: center;
    display: flex;
  }

  canvas {
    margin: auto;
    display: flex;
  }
</style>
