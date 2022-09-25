mod cell;
mod config;
mod creature;
mod utils;

use crate::config::Config;
use utils::set_panic_hook;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    set_panic_hook();
    let config = Config::create(500, 500);

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should habe a document on window");
    let body = document.body().expect("document should have a body");

    let canvas = document.create_element("canvas")?.dyn_into::<HtmlCanvasElement>()?;
    canvas.set_attribute(&"id", &"sim-canvas")?;
    canvas.set_width(config.get_width());
    canvas.set_height(config.get_height()); 
    let ctx = canvas.get_context("2d").unwrap().unwrap().dyn_into::<CanvasRenderingContext2d>()?;

    body.append_child(&canvas)?;

    Ok(())
}