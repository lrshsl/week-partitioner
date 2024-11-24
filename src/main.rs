use macroquad::{prelude::*, ui::root_ui};
use miniquad::{info, window::screen_size};

mod time;

mod table_context;
use table_context::{
    TableContext, BUTTON_FONT_SIZE, BUTTON_SIZE, DAY_END, DAY_START, DEFAULT_FONT_SIZE,
};

const THICK_LINES: f32 = 3.0;
const THIN_LINES: f32 = 1.0;

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

#[macroquad::main("Week partitioner")]
async fn main() {
    let mut table = TableContext::new();
    loop {
        clear_background(BLACK);
        if get_fps() < 50 {
            draw_text(&format!("FPS: {}", get_fps()), 20.0, 20.0, 30.0, DARKGRAY);
        }

        table.update(screen_size().into());

        {
            // Draw time axis
            let day_hours = (DAY_END - DAY_START).hours_precise();
            let hour_height = table.column_height / day_hours;

            for i in 0..(day_hours as i32 + 1) {
                let y = table.low_y + (i as f32) * hour_height;
                let h_str = format!("{}", DAY_START.hours() + i);
                let s = text_size(&h_str);
                draw_text(
                    &h_str,
                    table.low_x - s.x - 20.0,
                    y + s.y * 0.5,
                    DEFAULT_FONT_SIZE,
                    GRAY,
                );

                draw_line(table.low_x, y, table.high_x, y, THIN_LINES, DARKGRAY);
            }
        }

        {
            // Draw day tables
            for (i, day) in ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]
                .iter()
                .enumerate()
            {
                draw_day(&table, i, day);
            }
        }

        {
            let spacing = table.table_width / 3.0;
            for (i, (entry, color)) in [("Analysis", YELLOW), ("Phy", RED), ("LinAlg", GREEN)]
                .iter()
                .enumerate()
            {
                let x = table.low_x + spacing * i as f32 + spacing * 0.5;
                let y = table.high_y + 30.0;

                let w = BUTTON_SIZE.x;
                let h = BUTTON_SIZE.y;

                draw_rectangle_lines(x, y, w, h, 2.0, *color);
                draw_text_centered!(entry,
                    x + w * 0.5,
                    y + h * 0.5,
                    font_size: BUTTON_FONT_SIZE,
                    color: *color);

                if is_clicked(x, y, w, h) {
                    info!("Selected: {}", entry);
                }
            }
        }

        next_frame().await
    }
}

fn draw_text_centered(s: &str, x: f32, y: f32, font_size: f32, color: Color) {
    let c = get_text_center(s, None, font_size as u16, 1.0, 0.0).abs() * 2.0;
    draw_text(s, x - c.x * 0.5, y + c.y * 0.5, font_size, color);
}

fn is_clicked(x: f32, y: f32, w: f32, h: f32) -> bool {
    is_mouse_button_pressed(MouseButton::Left)
        && Rect { x, y, w, h }.contains(mouse_position().into())
}

fn draw_day(table: &TableContext, day_index: usize, name: &'static str) {
    // Draw name
    let column_x = table.low_x + table.column_width * (day_index as f32);
    draw_text_centered!(
        name,
        column_x + (table.column_width * 0.5),
        table.low_y - 10.0
    );

    // Draw box
    draw_rectangle_lines(
        column_x,
        table.low_y,
        table.column_width,
        table.column_height,
        THICK_LINES,
        WHITE,
    );
}

fn text_size(s: &str) -> Vec2 {
    get_text_center(s, None, DEFAULT_FONT_SIZE as u16, 1.0, 0.0).abs() * 2.0
}
