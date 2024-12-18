use crate::prelude::*;
use macroquad::text::get_text_center;

macro_rules! draw_text_centered {
    ($text:expr, $x:expr, $y:expr) => {
        draw_text_centered_fn($text, $x, $y, crate::DEFAULT_FONT_SIZE, crate::WHITE)
    };
    ($text:expr, $x:expr, $y:expr, color: $color:expr) => {
        draw_text_centered_fn($text, $x, $y, crate::DEFAULT_FONT_SIZE, $color)
    };
    ($text:expr, $x:expr, $y:expr, font_size: $font_size:expr) => {
        draw_text_centered_fn($text, $x, $y, $font_size, crate::WHITE)
    };
    ($text:expr, $x:expr, $y:expr, font_size: $font_size:expr, color: $color:expr) => {
        draw_text_centered_fn($text, $x, $y, $font_size, $color)
    };
}

pub(crate) use draw_text_centered;

pub fn draw_text_centered_fn(s: &str, x: f32, y: f32, font_size: f32, color: Color) {
    let c = get_text_center(s, None, font_size as u16, 1.0, 0.0).abs() * 2.0;
    draw_text(s, x - c.x * 0.5, y + c.y * 0.5, font_size, color);
}

pub const fn rect(x: f32, y: f32, w: f32, h: f32) -> Rect {
    Rect { x, y, w, h }
}

pub fn draw_rect_lines(r: Rect, thickness: f32, clr: Color) {
    draw_rectangle_lines(r.x, r.y, r.w, r.h, thickness, clr)
}

pub fn draw_rect(r: Rect, clr: Color) {
    draw_rectangle(r.x, r.y, r.w, r.h, clr)
}

pub fn is_clicked(rect: Rect) -> bool {
    is_mouse_button_pressed(MouseButton::Left)
        && rect.contains(to_buffer_cords(mouse_position().into()))
}

pub fn text_size(s: &str, font_size: f32) -> Vec2 {
    get_text_center(s, None, DEFAULT_FONT_SIZE as u16, 1.0, 0.0).abs() * 2.0
}

pub fn mouse_pos() -> Vec2 {
    to_buffer_cords(mouse_position().into())
}

pub fn to_buffer_cords(screen_cords: Vec2) -> Vec2 {
    screen_cords / Vec2::from(screen_size()) * AREA_SIZE
}
