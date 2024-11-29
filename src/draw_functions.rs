use macroquad::{
    math::Rect,
    shapes::draw_rectangle_lines,
    text::draw_text,
    texture::{draw_texture_ex, DrawTextureParams, Texture2D},
    time::get_fps,
    window::clear_background,
};

use crate::{
    prelude::*,
    tracks::{draw_tracks, get_field_rect},
    COLUMN_WIDTH, DAY_END, DAY_NAMES, DAY_START, HOUR_HEIGHT, LEGEND_SPACING, N_DAYS,
};

pub(crate) fn draw_screen(screen_buffer: &Texture2D) {
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

pub(crate) fn draw_all(ctx: &Context, track_buttons: &Vec<Button>) {
    clear_background(BLACK);
    draw_fps();

    draw_hours();
    draw_days(&DAY_NAMES);
    for button in track_buttons {
        button.draw();
    }
    draw_tracks(&ctx.fields);
}

pub(crate) fn draw_fps() {
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
    for h in 0..N_HOURS {
        let h_str = format!("{}", DAY_START.hours() + h as i32);
        let s = text_size(&h_str, SMALLER_FONT_SIZE);
        draw_text(
            &h_str,
            TABLE_MARGIN.x - s.x - vw(2.0),
            TABLE_MARGIN.y + COLUMN_WIDTH * h as f32 + s.y * 0.5,
            SMALLER_FONT_SIZE,
            GRAY,
        );
    }

    for field in 0..(N_HOURS * N_DAYS) {
        draw_rect_lines(get_field_rect(field), THIN_LINES, DARKGRAY);
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
