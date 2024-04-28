// Replace `wasm_rust_template` with whatever you name your crate.
// use wasm_rust_template::run;

// This will run when you do a `cargo run`.
fn main() {
    env_logger::init();

    /* `env_logger` decides whether to print this to stderr at runtime by 
     * looking at the `RUST_LOG` environment variable. The levels are trace, 
     * debug, info, warn, and error. The `RUST_LOG` level must be the same 
     * priority level or lower as the one you print with. So, to print the 
     * below message, you can run `RUST_LOG=debug cargo run`, or 
     * `RUST_LOG=trace cargo run`, but not info, warn, or error in this case. 
     * BTW, passing the crate name instead of a log level enables all levels. 
     * To learn more, take a look at https://docs.rs/env_logger. */
    log::info!("Hello from main!");
}
