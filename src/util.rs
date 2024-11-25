use crate::prelude::*;

#[macro_export]
macro_rules! draw_text_centered {
    ($text:expr, $x:expr, $y:expr) => {
        draw_text_centered($text, $x, $y, crate::DEFAULT_FONT_SIZE, crate::WHITE)
    };
    ($text:expr, $x:expr, $y:expr, color: $color:expr) => {
        draw_text_centered($text, $x, $y, crate::DEFAULT_FONT_SIZE, $color)
    };
    ($text:expr, $x:expr, $y:expr, font_size: $font_size:expr) => {
        draw_text_centered($text, $x, $y, $font_size, crate::WHITE)
    };
    ($text:expr, $x:expr, $y:expr, font_size: $font_size:expr, color: $color:expr) => {
        draw_text_centered($text, $x, $y, $font_size, $color)
    };
}

pub const fn rect(x: f32, y: f32, w: f32, h: f32) -> Rect {
    Rect { x, y, w, h }
}

pub fn draw_text_centered(s: &str, x: f32, y: f32, font_size: f32, color: Color) {
    let c = get_text_center(s, None, font_size as u16, 1.0, 0.0).abs() * 2.0;
    draw_text(s, x - c.x * 0.5, y + c.y * 0.5, font_size, color);
}

pub fn is_clicked(x: f32, y: f32, w: f32, h: f32) -> bool {
    is_mouse_button_pressed(MouseButton::Left)
        && Rect { x, y, w, h }.contains(mouse_position().into())
}
