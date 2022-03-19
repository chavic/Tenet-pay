extern crate lazy_static;

use app::App;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

mod app;
mod graphql;
mod utils;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    let app = App::new();
    App::start(app);
    Ok(())
}
