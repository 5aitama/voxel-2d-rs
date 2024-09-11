use nalgebra_glm::Vec2;
use web_sys::CanvasRenderingContext2d;

pub struct Context2D {
    ctx: CanvasRenderingContext2d,
}

impl Context2D {
    /// Create a new [2D context](Context2D).
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context.
    ///
    pub fn new(ctx: &CanvasRenderingContext2d) -> Self {
        Self { ctx: ctx.clone() }
    }

    /// Draw a box.
    ///
    /// # Arguments
    ///
    /// * `pos` - The position of the center of the box.
    /// * `size` - The size of the box *(not the extent !)*
    pub fn draw_box(&self, pos: &Vec2, size: f32, color: Option<String>) {
        self.ctx.begin_path();
        self.ctx.rect(
            (pos.x - size / 2.0) as f64,
            (pos.y - size / 2.0) as f64,
            size as f64,
            size as f64,
        );
        // self.ctx
        //     .set_fill_style(&color.unwrap_or("black".into()).into());
        // self.ctx.fill();
        self.ctx.set_stroke_style(&color.unwrap_or("black".into()).into());
        self.ctx.stroke();
    }

    /// Draw a line
    ///
    /// # Arguments
    ///
    /// * `from` - The position at where the line start.
    /// * `to` - The position at where the line end.
    ///
    pub fn draw_line(&self, from: &Vec2, to: &Vec2) {
        self.ctx.begin_path();
        self.ctx.move_to(from.x as f64, from.y as f64);
        self.ctx.line_to(to.x as f64, to.y as f64);
        self.ctx.set_stroke_style(&"black".into());
        self.ctx.stroke();
    }
}
