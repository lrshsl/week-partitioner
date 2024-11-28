use crate::{prelude::*, COLUMN_WIDTH, DAY_END, DAY_NAMES, DAY_START, LEGEND_SPACING, TRACKS};

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

pub(crate) fn draw_all(track_buttons: &Vec<Button>) {
    clear_background(BLACK);

    draw_hours();
    draw_days(&DAY_NAMES);
    for button in track_buttons {
        button.draw();
    }
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
