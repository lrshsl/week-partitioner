use crate::{prelude::*, Track};

pub(crate) struct Button {
    text: &'static str,
    clr: Color,
    rect: Rect,
}

impl Button {
    pub const fn new(text: &'static str, clr: Color, rect: Rect) -> Self {
        Self { text, clr, rect }
    }

    pub fn draw(&self) {
        let (x, y) = self.rect.point().into();
        let (w, h) = self.rect.size().into();
        draw_rectangle_lines(x, y, w, h, 5.0, self.clr);
        draw_text_centered!(self.text,
                            x + w * 0.5,
                            y + h * 0.5,
                            font_size: BUTTON_FONT_SIZE,
                            color: self.clr);
    }

    pub fn update(&self, ctx: &mut Context) {
        if is_mouse_button_pressed(MouseButton::Left) {}
        if is_clicked(self.rect) {
            ctx.current_track = Track::from((self.text, self.clr));
            println!("Selected: {}", ctx.current_track.name);
        }
    }
}
