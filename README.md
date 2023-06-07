# Oxidized Game of Life

## Intro

Oxidized Game of Life is an implementation of a finite-state cellular automaton in Rust, WASM, SvelteKit and TypeScript. The game, invented in 1970 by Cambridge mathematician John Conway, is a simulation of cells on a 2D grid. A finite set of rules determines whether a cell is dead or alive at any given tick in time. By porting over performance-critical methods to Rust, we achieved great performance gains compared to JavaScript-only. It's open-source and available on GitHub.

## Demo

You can find a live demo of this application [here](https://oxidizedgol.vercel.app/).

## Requirements

You should be familiar with the Rust WASM book and SvelteKit. You can find the Rust WASM book [here](https://rustwasm.github.io/docs/book/). You can find the SvelteKit documentation [here](https://kit.svelte.dev/docs). Make sure that you have the full dev toolchain for Rust installed on your machine as per the Rust WASm documentation. The same goes for SvelteKit and its dependencies.

Overall, the learnings from the project setup here have culminated in a starter template for Rust WASM and SvelteKit. You can find it [here](https://github.com/tobiplay/rust-wasm-sveltekit-starter). It's a great starting point for your own projects in the future and utilizes many of the same tools and design choices as this project. 

You'll also find a lot more information on developing with Rust, WASM, and SvelteKit, together with Vite as the bundler in this setup in the starter template. This `README.md` will only cover the basics.

## Development

After cloning the repo, you can navigate to the `svelte/` directory and run `npm install` to install all dependencies. Then, you can run `npm run dev` to start the development server. The application will be available at `localhost:3000`.

When you make changes ot the Rust core of the application, you'll have to rebuild the module. You can refer to the `build.sh` script in the root directory of the project. It will build the Rust module and copy it to the correct location in the SvelteKit project. You can run it with `./build.sh`. Also, you can execute the script content manually by just copying the commands and pasting them one by one into your terminal.

We utilize GitHub Actions to build and run tests. You can find the workflow file in `.github/workflows/`.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.