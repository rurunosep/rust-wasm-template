# Rust WASM Template

Simple template for a project using Rust compiled to WebAssembly interoperating with JavaScript.

Requires Rust and wasm-pack installed.

- `/rust`: the Rust package that will be compiled to a WebAssembly module by wasm-pack. Comes with the following crate dependecies:
  - `wasm-bindgen`: import JavaScript things into Rust and export Rust things to JavaScript
  - `js_sys`: bindings to JavaScript's standard objects and methods
  - `web_sys`: bindings to the browser's Web APIs
  - `console_error_panic_hook`: print Rust panics to console.error
  - `wee_alloc`: tiny allocator designed for WASM
- `js/index.js`: the entry point of the app which will import and use the WebAssembly module
- `index.html`: the HTML template into which the bundled JavaScript will be inserted by Webpack
- `/static`: static files that will be copied into the output directory by Webpack
- `server.js`: minimal static Express server to serve files from the output directory

- `webpack.config.js`
  - `WasmPackPlugin`: will compile Rust using wasm-pack
  - `HtmlWebpackPlugin`: will insert the bundled JavaScript into the HTML template
  - `CopyWebpackPlugin`: will copy static files to output directory
- `package.json`

  - `build`: clear output directories and rebuild app
  - `start`: start minimal static server
  - `dev`: start Webpack dev server with live reloading (with a development build)
  - `prod`: start Webpack dev server with live reloading (with an optimized, production build)

- `/dist` _(generated)_: the final build output directory from which files are served
- `/wasm` _(generated)_: the destination of the compiled WASM module which is imported by JavaScript
