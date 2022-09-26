mod cell;
mod config;
mod creature;
mod manager;
mod utils;

use crate::config::Config;
use manager::Manager;
use wasm_bindgen::{prelude::*};
use web_sys::{CanvasRenderingContext2d};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let manager = Manager::init();

    draw(manager.get_canvas_context(), manager.get_config());

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