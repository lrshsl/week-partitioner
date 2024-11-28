use macroquad::math::{vec2, Vec2};

pub(crate) const W: f32 = 700.0;
pub(crate) const H: f32 = 700.0;
pub(crate) const AREA_SIZE: Vec2 = vec2(W, H);
pub(crate) const RESOLUTION: Vec2 = vec2(1920.0, 1080.0);

pub(crate) const MIN_CANVAS_SIZE: Vec2 = vec2(600.0, 400.0);

pub(crate) const TABLE_MARGIN: Vec2 = vec2(vw(10.0), vh(12.0));
pub(crate) const TABLE_SIZE: Vec2 = vec2(
    vw(100.0) - 2.0 * TABLE_MARGIN.x,
    vh(100.0) - 2.0 * TABLE_MARGIN.y,
);

pub(crate) const DEFAULT_FONT_SIZE: f32 = 30.0;
pub(crate) const BUTTON_FONT_SIZE: f32 = 20.0;

pub(crate) const BUTTON_SIZE: Vec2 = vec2(100.0, 30.0);

pub(crate) const THICK_LINES: f32 = 3.0;
pub(crate) const THIN_LINES: f32 = 1.0;

pub const fn vw(w: f32) -> f32 {
    w * W / 100.0
}

pub const fn vh(h: f32) -> f32 {
    h * H / 100.0
}
