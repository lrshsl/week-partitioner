use std::sync::Mutex;

use miniquad::window::screen_size;
use prelude::*;

mod constants;
mod prelude;
mod table_context;
mod time;
mod util;

pub(crate) const DAY_START: Time = hours(6);
pub(crate) const DAY_END: Time = hours(21);

pub(crate) const N_ENTRIES: usize = 3;
pub(crate) const ENTRIES: [(&'static str, Color); N_ENTRIES] =
    [("Analysis", YELLOW), ("Phy", RED), ("LinAlg", GREEN)];
pub(crate) const LEGEND_SPACING: f32 = TABLE_SIZE.x / N_ENTRIES as f32;

pub(crate) const COLUMN_WIDTH: f32 = TABLE_SIZE.x / 7.0;

pub(crate) const N_DAYS: usize = 7;
pub(crate) const DAY_NAMES: [&'static str; N_DAYS] =
    ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

#[macroquad::main("Week partitioner")]
async fn main() {
    let mut redraw_required = true;

    let render_target = render_target(RESOLUTION.x as u32, RESOLUTION.y as u32);

    let screen_buffer = Camera2D {
        zoom: Vec2::ONE / AREA_SIZE * 2.0,
        offset: Vec2::NEG_ONE,
        render_target: Some(render_target),
        ..Default::default()
    };

    loop {
        // Draw to texture
        if redraw_required {
            set_camera(&screen_buffer);
            draw();
            set_default_camera();
        }

        // Draw texture to screen
        draw_screen(&screen_buffer.render_target.as_ref().unwrap().texture);

        next_frame().await
    }
}

fn draw_screen(screen_buffer: &Texture2D) {
    draw_texture_ex(
        screen_buffer,
        0.0,
        0.0,
        WHITE,
        DrawTextureParams {
            source: Some(Rect::new(0.0, 0.0, RESOLUTION.x, RESOLUTION.y)),
            dest_size: Some(screen_size().into()),
            ..Default::default()
        },
    );
}

fn draw() {
    clear_background(BLACK);
    draw_fps();

    draw_hours();
    draw_days(&DAY_NAMES);
    draw_legend(&ENTRIES);
}

fn draw_fps() {
    draw_text(
        &format!("FPS: {}", get_fps()),
        vw(5.0),
        vh(5.0),
        DEFAULT_FONT_SIZE,
        DARKGRAY,
    );
}

fn draw_hours() {
    // Draw time axis
    let day_hours = (DAY_END - DAY_START).hours_precise();
    let hour_height = TABLE_SIZE.y / day_hours;

    for i in 0..(day_hours as i32 + 1) {
        let y = TABLE_MARGIN.y + (i as f32) * hour_height;
        let h_str = format!("{}", DAY_START.hours() + i);
        let s = text_size(&h_str);
        draw_text(
            &h_str,
            TABLE_MARGIN.x - s.x - vw(20.0),
            y + s.y * 0.5,
            DEFAULT_FONT_SIZE,
            GRAY,
        );

        draw_line(
            TABLE_MARGIN.x,
            y,
            TABLE_MARGIN.x + TABLE_SIZE.x,
            y,
            THIN_LINES,
            DARKGRAY,
        );
    }
}

fn draw_days(day_names: &[&'static str]) {
    // Draw day tables
    for (i, day_name) in day_names.iter().enumerate() {
        // Draw name
        let column_x = TABLE_MARGIN.x + COLUMN_WIDTH * (i as f32);
        draw_text_centered!(
            day_name,
            column_x + (COLUMN_WIDTH * 0.5),
            TABLE_MARGIN.y - vh(3.00)
        );

        // Draw box
        draw_rectangle_lines(
            column_x,
            TABLE_MARGIN.y,
            COLUMN_WIDTH,
            TABLE_SIZE.y,
            THICK_LINES,
            WHITE,
        );
    }
}

fn draw_legend(entries: &[(&'static str, Color)]) {
    for (i, (entry, color)) in entries.iter().enumerate() {
        let x = TABLE_MARGIN.x + LEGEND_SPACING * i as f32 + LEGEND_SPACING * 0.5;
        let y = H - vh(8.0);

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

fn text_size(s: &str) -> Vec2 {
    get_text_center(s, None, DEFAULT_FONT_SIZE as u16, 1.0, 0.0).abs() * 2.0
}
