use std::collections::HashMap;

use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlCanvasElement, HtmlElement, CanvasRenderingContext2d};

use crate::{config::Config, cell::Cell, creature::Creature, utils::set_panic_hook};

pub struct Manager {
    canvas: HtmlCanvasElement,
    cells: HashMap<u32, Cell>,
    config: Config,
    creatures: Vec<Creature>,
    ctx: CanvasRenderingContext2d,
    debug_bar: HtmlElement,
    document: Document
}

impl Manager {
    pub fn init() -> Self {
        let config = Config::create(500, 500, 10, 10, true);

        if config.is_debug() {
            set_panic_hook();
        }

        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should habe a document on window");

        let body = document.body().expect("document should have a body");
        let canvas = document.create_element("canvas").expect("document should be able to create a canvas element").dyn_into::<HtmlCanvasElement>().expect("canvas should be castable to HtmlCanvasElement");
        canvas.set_attribute(&"id", &"sim-canvas").expect("canvas should be able to set an id");
        canvas.set_width(config.get_width());
        canvas.set_height(config.get_height()); 
        let ctx = canvas.get_context("2d").unwrap().expect("canvas should have a 2d context").dyn_into::<CanvasRenderingContext2d>().expect("2d context should be castable to CanvasRenderingContext2d");

        let canvas_wrapper = document.create_element("section").expect("document should be able to create a section element").dyn_into::<HtmlElement>().expect("section should be castable to HtmlElement");
        canvas_wrapper.set_class_name("flex-row");
        canvas_wrapper.append_child(&canvas).expect("canvas_wrapper should be able to have a child of canvas");

        body.append_child(&canvas_wrapper).expect("document body should be able to have a child of canvas_wrapper");

        let debug_bar = document.create_element("section").expect("document should be able to create a section element").dyn_into::<HtmlElement>().expect("section should be castable to HtmlElement");
        if config.is_debug() {
            debug_bar.set_class_name("flex-row");
            debug_bar.set_inner_text("DEBUG BAR");
            
            body.append_child(&debug_bar).expect("document body should be able to have a child of debug_bar");
        }

        Self {
            canvas,
            cells: todo!(),
            config,
            creatures: todo!(),
            ctx,
            debug_bar,
            document,
        }
    }

    pub fn get_config(&self) -> &Config {
        &self.config
    }

    pub fn get_canvas_context(&self) -> &CanvasRenderingContext2d {
        &self.ctx
    }
}