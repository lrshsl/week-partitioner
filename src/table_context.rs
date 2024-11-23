use macroquad::math::Vec2;

use crate::time::{hours, Time};

pub(crate) const MIN_TABLE_WIDTH: f32 = 600.0;
pub(crate) const MIN_TABLE_HEIGHT: f32 = 400.0;
pub(crate) const TABLE_X_MARGIN: f32 = 0.1;
pub(crate) const TABLE_Y_MARGIN: f32 = 0.12;
pub(crate) const TABLE_WIDTH: f32 = 1.0 - 2.0 * TABLE_X_MARGIN;
pub(crate) const TABLE_HEIGHT: f32 = 1.0 - 2.0 * TABLE_Y_MARGIN;

pub(crate) const FONT_SIZE: f32 = 30.0;

pub(crate) const DAY_START: Time = hours(6);
pub(crate) const DAY_END: Time = hours(21);

pub(crate) struct TableContext {
    pub table_width: f32,
    pub table_height: f32,

    pub column_width: f32,
    pub column_height: f32,

    pub low_x: f32,
    pub low_y: f32,
    pub high_x: f32,
    pub high_y: f32,
}

impl TableContext {
    pub fn new() -> Self {
        Self {
            table_width: 0.0,
            table_height: 0.0,
            column_width: 0.0,
            column_height: 0.0,
            low_x: 0.0,
            low_y: 0.0,
            high_x: 0.0,
            high_y: 0.0,
        }
    }

    pub fn update(&mut self, screen_size: Vec2) {
        self.table_width = (TABLE_WIDTH * screen_size.x).max(MIN_TABLE_WIDTH);
        self.table_height = (TABLE_HEIGHT * screen_size.y).max(MIN_TABLE_HEIGHT);

        self.column_width = self.table_width / 7.0;
        self.column_height = self.table_height;

        self.low_x = TABLE_X_MARGIN * screen_size.x;
        self.low_y = TABLE_Y_MARGIN * screen_size.y;

        self.high_x = self.low_x + self.table_width;
        self.high_y = self.low_y + self.table_height;
    }
}
