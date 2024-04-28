My template for using Rust on the frontend.

It's set up to be able to be run natively and through WASM.
The rationale behind this is that if you want to, say, use [`wgpu`](https://docs.rs/wgpu/latest/wgpu/) to render to a canvas in WASM, you'll be able to use the same Rust code to render the same image to a window with Metal/Vulkan/DX12 when you run it natively, instead of rendering to a webview on the desktop.
Architecting your project this way will make it easier to reuse code across platforms, allowing you to get much better performance for free.

## Setup
In order to run Rust in the browser, you need to do two things; compile it for the `wasm32` architecture, and generate some JS glue code to be able to easily use the wasm binary (for example, initialisation of the linear memory buffer, methods for (en|de)coding strings for passing them to WASM, and other general runtime memory management).

### The easy way
[wasm-pack](https://github.com/rustwasm/wasm-pack) is a good tool for cross-compiling & testing Rust projects for the `wasm32` architecture.

To install it, run `cargo install wasm-pack`.

### The explicit way
At its core, `wasm-pack build` just wraps two commands; `cargo build --target=wasm32-unknown-unknown` and `wasm-bindgen --out-dir ./pkg out.wasm`

```bash
cargo build --target=wasm32-unknown-unknown && wasm-bindgen --out-dir ./pkg ./target/wasm32-unknown-unknown/debug/wasm-rust-template.wasm
```

## Running
You can run this crate natively, by simply running `cargo run`. The entrypoint will be `src/main.rs:main`.  

The WASM entrypoint will be `src/lib.rs:run`.
To run it, first build it (as described above), and then run Vite to serve `www`.

I quite like [Bun](https://bun.sh), so I run `bun --cwd www dev`. If you just want to use `npm`, run `cd www && npm run dev`.


### Demo

The demo page that is served should look like this. The button calls a Rust function to update the `textContent` of the button.

![image](https://github.com/malted/wasm-rust-template/assets/59726149/90356b12-1567-4678-adc1-44e5eccae5fc)
