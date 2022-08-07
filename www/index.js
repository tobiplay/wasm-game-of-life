import * as wasm from "wasm-game-of-life";

wasm.greet();

var button = document.createElement("button");
button.innerHTML = "Do Something";

var body = document.getElementsByTagName("body")[0];
body.appendChild(button);

button.addEventListener("click", wasm.test);
