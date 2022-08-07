import * as wasm from "wasm-game-of-life";
// Specifically import the Universe struct:
import { Universe } from "wasm-game-of-life";

// Grab the <pre> element from the DOM:
const pre = document.getElementById("game-of-life-canvas");
// Init a new Universe by calling the specifc constructor
// function that we built in Rust and compiled to WASM:
const universe = Universe.new();

// The JavaScript portion of our program runs in a
// requestAnimationFrame loop. On each iteration,
// it draws the current universe to the <pre>, and then calls
// Universe::tick to advance one tick in time.
const renderLoop = () => {
    pre.textContent = universe.render();
    universe.tick();

    requestAnimationFrame(renderLoop);
};

// To start the rendering process, all we have to do is
// make the initial call for the first iteration of the
// rendering loop:
requestAnimationFrame(renderLoop);

// This code is just meant as means to debug
// the JavaScript wasm_bindgen interface for now.
// wasm.greet();

// var button = document.createElement("button");
// button.innerHTML = "Do Something";

// var body = document.getElementsByTagName("body")[0];
// body.appendChild(button);

// button.addEventListener("click", () => {
//     requestAnimationFrame(renderLoop);
// });
