#[cfg(feature = "js")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "js")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn console_log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = error)]
    fn console_error(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = warn)]
    fn console_warn(s: &str);
}

#[cfg(feature = "js")]
pub fn log(s: &str) {
    console_log(s);
}

#[cfg(not(feature = "js"))]
pub fn log(s: &str) {
    println!("{}", s);
}

#[cfg(feature = "js")]
pub fn error(s: &str) {
    console_error(s);
}

#[cfg(not(feature = "js"))]
pub fn error(s: &str) {
    eprintln!("{}", s);
}

#[cfg(feature = "js")]
pub fn warn(s: &str) {
    console_warn(s);
}

#[cfg(not(feature = "js"))]
pub fn warn(s: &str) {
    eprintln!("{}", s);
}
