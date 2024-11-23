use macroquad::prelude::*;

#[macroquad::main("Week partitioner")]
async fn main() {
    loop {
        clear_background(BLACK);
        draw_text(&format!("FPS: {}", get_fps()), 20.0, 20.0, 30.0, DARKGRAY);

        for (i, day) in ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]
            .iter()
            .enumerate()
        {
            draw_day(i, day);
        }

        next_frame().await
    }
}

const TABLE_X_MARGIN: f32 = 0.1;
const TABLE_Y_MARGIN: f32 = 0.12;
const TABLE_WIDTH: f32 = 1.0 - 2.0 * TABLE_X_MARGIN;
const TABLE_HEIGHT: f32 = 1.0 - 2.0 * TABLE_Y_MARGIN;

const FONT_SIZE: f32 = 30.0;

struct Time {
    minutes: i32,
}

impl std::ops::Add<Time> for Time {
    type Output = Self;

    fn add(self, rhs: Time) -> Self {
        Self {
            minutes: self.minutes + rhs.minutes,
        }
    }
}

impl std::ops::Sub<Time> for Time {
    type Output = Self;

    fn sub(self, rhs: Time) -> Self::Output {
        Self {
            minutes: self.minutes - rhs.minutes,
        }
    }
}

impl Time {
    pub const fn from_hours(hours: i32) -> Self {
        Self {
            minutes: hours * 60,
        }
    }

    pub const fn from_minutes(minutes: i32) -> Self {
        Self { minutes }
    }

    pub const fn hours_precise(&self) -> f32 {
        (self.minutes as f32) / 60.0
    }

    pub const fn hours(&self) -> i32 {
        self.minutes / 60
    }

    pub const fn minutes(&self) -> i32 {
        self.minutes
    }
}
const fn hours(h: i32) -> Time {
    Time::from_hours(h)
}
const fn minutes(m: i32) -> Time {
    Time::from_minutes(m)
}

const DAY_START: Time = hours(6);
const DAY_END: Time = hours(21);

fn draw_day(day_index: usize, name: &'static str) {
    let column_width = TABLE_WIDTH / 7.0 * screen_width();
    let column_height = TABLE_HEIGHT * screen_height();

    let low_x = TABLE_X_MARGIN * screen_width() + column_width * (day_index as f32);
    let low_y = TABLE_Y_MARGIN * screen_height();
    let high_x = low_x + column_width;

    // Draw name
    let text_center = get_text_center(name, None, FONT_SIZE as u16, 1.0, 0.0);
    draw_text(
        name,
        low_x + (column_width * 0.5) - text_center.x,
        low_y - 10.0,
        FONT_SIZE,
        WHITE,
    );

    // Draw box
    draw_rectangle_lines(
        low_x,
        low_y,
        column_width,
        TABLE_HEIGHT * screen_height(),
        2.0,
        WHITE,
    );

    // Draw hours
    let day_hours = (DAY_END - DAY_START).hours_precise();
    let hour_height = column_height / day_hours;
    let hours = DAY_START.hours()..DAY_END.hours();
    for h in hours {
        let y = low_y + ((h - DAY_START.hours()) as f32) * hour_height;
        draw_line(low_x, y, high_x, y, 1.0, DARKGRAY);
    }
}
