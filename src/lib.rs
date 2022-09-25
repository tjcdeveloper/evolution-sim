mod cell;
mod config;
mod creature;
mod utils;

use crate::config::Config;
use utils::set_panic_hook;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlElement};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let config = Config::create(500, 500, 10, 10, true);

    if config.is_debug() {
        set_panic_hook();
    }

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should habe a document on window");
    let body = document.body().expect("document should have a body");
    let canvas = document.create_element("canvas")?.dyn_into::<HtmlCanvasElement>()?;
    canvas.set_attribute(&"id", &"sim-canvas")?;
    canvas.set_width(config.get_width());
    canvas.set_height(config.get_height()); 
    let ctx = canvas.get_context("2d").unwrap().unwrap().dyn_into::<CanvasRenderingContext2d>()?;

    let canvas_wrapper = document.create_element("section")?.dyn_into::<HtmlElement>()?;
    canvas_wrapper.set_class_name("flex-row");
    canvas_wrapper.append_child(&canvas)?;

    body.append_child(&canvas_wrapper)?;

    if config.is_debug() {
        let debug_bar = document.create_element("section")?.dyn_into::<HtmlElement>()?;
        debug_bar.set_class_name("flex-row");
        debug_bar.set_inner_text("DEBUG BAR");
        
        body.append_child(&debug_bar)?;
    }

    draw(&ctx, &config);

    Ok(())
}

fn draw(ctx: &CanvasRenderingContext2d, config: &Config) {
    ctx.clear_rect(0f64, 0f64, config.get_width() as f64, config.get_height() as f64);

    for x in 1..config.get_num_cells_x() {
        let y = f64::from(x * config.get_cell_height());
        ctx.move_to(0f64, y);
        ctx.line_to(config.get_width() as f64, y);
    }

    for y in 1..config.get_num_cells_y() {
        let x = f64::from(y * config.get_cell_width());
        ctx.move_to(x, 0f64);
        ctx.line_to(x, config.get_height() as f64);
    }

    ctx.set_stroke_style(&JsValue::from_str("black"));
    ctx.stroke();
}