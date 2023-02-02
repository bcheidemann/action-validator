#[cfg(feature = "js")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "js")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = error)]
    fn console_error(s: &str);
}

#[cfg(feature = "js")]
pub fn error(s: &str) {
    console_error(s);
}

#[cfg(not(feature = "js"))]
pub fn error(s: &str) {
    eprintln!("{}", s);
}
