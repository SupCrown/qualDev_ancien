# Social Simulation base project, derived from the eframe template

This base project displays some moving circles in a UI. 

The aim of the associated lab is to improve the quality of this code in terms of:
- Readability
- Maintainability
- Testability

Once the running code is in a satisfying state, the aim is to add new features to the simulation.


## Getting started

### Testing on your machine, natively

You should be able to run the program with :

`cargo run --release`

### Running in a web container, locally

You can compile this app to [WASM](https://en.wikipedia.org/wiki/WebAssembly) and publish it as a web page.

We use [Trunk](https://trunkrs.dev/) to build for web target.
1. Install the required target with `rustup target add wasm32-unknown-unknown`.
2. Install Trunk with `cargo install --locked trunk`.
3. Run `trunk serve` to build and serve on `http://127.0.0.1:8080`. Trunk will rebuild automatically if you edit the project.
4. Open `http://127.0.0.1:8080/index.html#dev` in a browser. See the warning below.

> `assets/sw.js` script will try to cache our app, and loads the cached version when it cannot connect to server allowing your app to work offline (like PWA).
> appending `#dev` to `index.html` will skip this caching, allowing us to load the latest builds during development.

## Basic structure of the project

### app.rs

The part you'll adapt and extend is in `src/app.rs`. This is where you define your app's data and behavior.

### Others files

- main.rs: contains the main function and the setup of the app.
- lib.rs: 

## Learning about egui

`src/app.rs` contains a simple example app. This is just to give some inspiration - most of it can be removed if you like.

The official egui docs are at <https://docs.rs/egui>. If you prefer watching a video introduction, check out <https://www.youtube.com/watch?v=NtUkr_z7l84>. For inspiration, check out the [the egui web demo](https://emilk.github.io/egui/index.html) and follow the links in it to its source code.
