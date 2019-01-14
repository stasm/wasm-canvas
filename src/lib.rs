use std::f64;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
pub struct Game {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    ctx: CanvasRenderingContext2d,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement, ctx: CanvasRenderingContext2d) -> Game {
        Game {
            x: 100,
            y: 100,
            width: canvas.width(),
            height: canvas.height(),
            ctx,
        }
    }

    pub fn tick(&mut self, arrow_up: bool, arrow_right: bool, arrow_down: bool, arrow_left: bool) {
        if arrow_up {
            self.y = (self.height + self.y - 1) % self.height;
        }
        if arrow_right {
            self.x = (self.x + 1) % self.width;
        }
        if arrow_down {
            self.y = (self.y + 1) % self.height;
        }
        if arrow_left {
            self.x = (self.width + self.x - 1) % self.width;
        }

        self.ctx
            .clear_rect(0., 0., f64::from(self.width), f64::from(self.height));

        self.ctx
            .fill_rect(f64::from(self.x), f64::from(self.y), 10., 10.);
    }
}
