use macroquad::math::Vec2;

use crate::time::{hours, Time};

pub(crate) const TABLE_X_MARGIN: f32 = 0.1;
pub(crate) const TABLE_Y_MARGIN: f32 = 0.12;
pub(crate) const TABLE_WIDTH: f32 = 1.0 - 2.0 * TABLE_X_MARGIN;
pub(crate) const TABLE_HEIGHT: f32 = 1.0 - 2.0 * TABLE_Y_MARGIN;

pub(crate) const FONT_SIZE: f32 = 30.0;

pub(crate) const DAY_START: Time = hours(6);
pub(crate) const DAY_END: Time = hours(21);

pub(crate) struct TableContext {
    pub column_width: f32,
    pub column_height: f32,

    pub low_x: f32,
    pub low_y: f32,
    pub high_x: f32,
    pub high_y: f32,
}

impl TableContext {
    pub fn new(screen_size: Vec2) -> Self {
        let column_width = TABLE_WIDTH / 7.0 * screen_size.x;
        let column_height = TABLE_HEIGHT * screen_size.y;

        let low_x = TABLE_X_MARGIN * screen_size.x;
        let low_y = TABLE_Y_MARGIN * screen_size.y;
        let high_x = low_x + TABLE_WIDTH * screen_size.x;
        let high_y = low_y + TABLE_HEIGHT * screen_size.y;

        Self {
            column_width,
            column_height,
            low_x,
            low_y,
            high_x,
            high_y,
        }
    }
    pub fn update(&mut self, screen_size: Vec2) {
        self.column_width = TABLE_WIDTH / 7.0 * screen_size.x;
        self.column_height = TABLE_HEIGHT * screen_size.y;

        self.low_x = TABLE_X_MARGIN * screen_size.x;
        self.low_y = TABLE_Y_MARGIN * screen_size.y;
        self.high_x = self.low_x + TABLE_WIDTH;
        self.high_y = self.low_y + TABLE_HEIGHT;
    }
}
