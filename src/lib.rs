pub mod graphics;
pub mod traversal;

use graphics::context2d::Context2D;
use nalgebra_glm::Vec2;
use traversal::{cpu::traverse, ray::Ray, voxel::Voxel};
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, Window};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    fn requestAnimationFrame(callback: &Closure<dyn FnMut()>);
}

#[wasm_bindgen]
pub struct Test {
    ctx: CanvasRenderingContext2d,
    context: Context2D,
    canvas_element: HtmlCanvasElement,
    window: Window,
}

#[wasm_bindgen]
impl Test {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Test, JsValue> {
        let window = web_sys::window().ok_or("Failed to retrieve window")?;
        let document = window.document().ok_or("Failed to retrieve the document")?;

        let canvas_element = document
            .query_selector("#canvas")?
            .ok_or("Failed to retrieve the canvas element")?
            .dyn_into::<HtmlCanvasElement>()?;

        let ctx = canvas_element
            .get_context("2d")?
            .ok_or("Failed to retrieve context 2D")?
            .dyn_into::<CanvasRenderingContext2d>()?;

        let context = Context2D::new(&ctx);

        Ok(Self {
            window,
            canvas_element,
            ctx,
            context,
        })
    }

    pub fn render(&self, ox: f32, oy: f32, mouse_x: f32, mouse_y: f32) {
        let dpi = self.window.device_pixel_ratio() as u32;

        let new_w = self.canvas_element.client_width() as u32 * dpi;
        let new_h = self.canvas_element.client_height() as u32 * dpi;

        let mut w = self.canvas_element.width();
        let mut h = self.canvas_element.height();

        // Constantly check if the canvas size has changed.
        // If it was changed we resize the internal canvas
        // size.
        if new_w != w || new_h != h {
            self.canvas_element.set_width(new_w);
            self.canvas_element.set_height(new_h);

            w = new_w;
            h = new_h;
        }

        self.ctx.save();

        self.ctx.clear_rect(0.0, 0.0, w.into(), h.into());
        self.ctx.translate(0.0, h.into()).unwrap();
        self.ctx.scale(1.0, -1.0).unwrap();

        let beg = Vec2::new(ox, oy);
        let end = Vec2::new(mouse_x, mouse_y);
        let dir = (end - beg).normalize();

        let s = w.min(h) as f32 - 100.0 * dpi as f32;

        let ray = Ray::new(beg, dir);
        let voxel = Voxel::new(Vec2::new(w as f32 / 2.0, h as f32 / 2.0), s);
        traverse(&ray, voxel, &self.context);

        self.context.draw_line(&beg, &end);

        self.ctx.restore();
    }
}
