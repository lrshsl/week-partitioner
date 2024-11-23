use macroquad::prelude::*;

mod time;
use miniquad::window::screen_size;

mod table_context;
use table_context::{TableContext, DAY_END, DAY_START, FONT_SIZE, TABLE_HEIGHT};

const THICK_LINES: f32 = 3.0;
const THIN_LINES: f32 = 1.0;

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
                    FONT_SIZE,
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

        next_frame().await
    }
}

fn draw_day(table: &TableContext, day_index: usize, name: &'static str) {
    // Draw name
    let text_size = text_size(name);
    let column_x = table.low_x + table.column_width * (day_index as f32);
    draw_text(
        name,
        column_x + (table.column_width * 0.5) - text_size.x * 0.5,
        table.low_y - 10.0,
        FONT_SIZE,
        WHITE,
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
    get_text_center(s, None, FONT_SIZE as u16, 1.0, 0.0).abs() * 2.0
}
