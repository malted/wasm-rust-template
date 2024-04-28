cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        use wasm_bindgen::prelude::*;

        // "Importing" a JS method
        #[wasm_bindgen]
        extern {
            fn alert(s: &str);
        }

        // "Exporting" a Rust function
        #[wasm_bindgen]
        pub fn greet(name: &str) {
            alert(&format!("Hello, {}!", name));
        }

        #[wasm_bindgen]
        pub fn double_count(button: web_sys::HtmlButtonElement) {
            log::debug!("Count button clicked!");

            // Get the data-count HTML attribute.
            let mut count = button
                .dataset()
                .get("count")
                .unwrap_or("1".to_string())
                .parse::<u128>()
                .unwrap_or(1);

            // Perform some expensive operation.
            count *= 2;

            // Set data-count with the new count.
            button
                .dataset()
                .set("count", &count.to_string())
                .expect("Set the data-count HTML attribute");

            // Update the textContent.
            button.set_text_content(Some(&format!("The count is {count}")));
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    #[cfg(target_arch = "wasm32")]
    // Make sure your browser's devtools console log level is set to "verbose".
    console_log::init_with_level(log::Level::Debug).unwrap();

    log::info!("Hello from Rust!");
}
