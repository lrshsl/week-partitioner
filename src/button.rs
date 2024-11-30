use crate::{prelude::*, TrackId};

pub(crate) struct TrackButton {
    pub id: TrackId,
    button: Button,
}

impl TrackButton {
    pub const fn new(
        id: TrackId,
        text: &'static str,
        rect: Rect,
        fg_clr: Color,
        bg_clr: Color,
    ) -> Self {
        Self {
            id,
            button: Button::new(text, rect, fg_clr, bg_clr),
        }
    }

    pub fn draw(&self, ctx: &Context) {
        if ctx.current_track == Some(self.id) {
            self.button.draw_clr(self.button.fg_clr, self.button.bg_clr);
        } else {
            self.button.draw();
        }
    }

    pub fn is_clicked(&self) -> bool {
        self.button.is_clicked()
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Button {
    text: &'static str,
    rect: Rect,
    fg_clr: Color,
    bg_clr: Color,
}

impl Button {
    pub const fn new(text: &'static str, rect: Rect, fg_clr: Color, bg_clr: Color) -> Self {
        Self {
            text,
            rect,
            fg_clr,
            bg_clr,
        }
    }

    pub fn draw(&self) {
        self.draw_clr(self.bg_clr, self.fg_clr);
    }

    pub fn draw_clr(&self, bg_clr: Color, fg_clr: Color) {
        draw_rect(self.rect, bg_clr);
        draw_rect_lines(self.rect, 5.0, fg_clr);
        draw_text_centered!(self.text,
                            self.rect.x + self.rect.w * 0.5,
                            self.rect.y + self.rect.h * 0.5,
                            font_size: SMALLER_FONT_SIZE,
                            color: fg_clr);
    }

    pub fn is_clicked(&self) -> bool {
        is_clicked(self.rect)
    }
}
