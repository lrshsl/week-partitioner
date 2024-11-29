use crate::drag_state::update_drag_state;
use draw_functions::{draw_all, draw_fps, draw_screen};
use macroquad::{
    camera::{set_camera, set_default_camera, Camera2D},
    math::Rect,
    texture::render_target,
    window::next_frame,
};
use prelude::*;
use tracks::update_tracks;

mod button;
mod constants;
mod drag_state;
mod draw_functions;
mod prelude;
mod table_context;
mod time;
mod tracks;
mod util;

pub(crate) const DAY_START: Time = hours(6);
pub(crate) const DAY_END: Time = hours(21);
pub(crate) const N_HOURS: usize = (DAY_END.hours() - DAY_START.hours()) as usize;
pub(crate) const HOUR_HEIGHT: f32 = TABLE_SIZE.y / N_HOURS as f32;

pub(crate) const N_TRACKS: usize = 3;
pub(crate) const TRACKS: [(&'static str, Color); N_TRACKS] =
    [("Analysis II", RED), ("Algs", GREEN), ("LinAlg", YELLOW)];
pub(crate) const LEGEND_SPACING: f32 = TABLE_SIZE.x / N_TRACKS as f32;

pub(crate) const COLUMN_WIDTH: f32 = TABLE_SIZE.x / N_DAYS as f32;

pub(crate) const N_DAYS: usize = 7;
pub(crate) const DAY_NAMES: [&'static str; N_DAYS] =
    ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

#[macroquad::main("Week partitioner")]
async fn main() {
    let render_target = render_target(RESOLUTION.x as u32, RESOLUTION.y as u32);

    println!("margin: {}", TABLE_MARGIN);

    let screen_buffer = Camera2D {
        zoom: Vec2::ONE / AREA_SIZE * 2.0,
        offset: Vec2::NEG_ONE,
        render_target: Some(render_target),
        ..Default::default()
    };

    let mut ctx = Context {
        screen_size: vec2(0.0, 0.0),
        drag_state: None,

        current_track: Track::from(TRACKS[0]),
        fields: vec![None; N_DAYS * N_HOURS],
    };

    let track_buttons = make_track_buttons();

    loop {
        #[cfg(debug_assertions)]
        if is_mouse_button_pressed(MouseButton::Middle) {
            println!(
                "Screen pos {:?} -> buffer pos {:?}",
                mouse_position(),
                mouse_pos()
            );
        }

        // Draw to texture
        update_all(&mut ctx, &track_buttons);
        set_camera(&screen_buffer);
        draw_all(&ctx, &track_buttons);

        // Draw texture to screen
        set_default_camera();
        draw_screen(&screen_buffer.render_target.as_ref().unwrap().texture);

        next_frame().await
    }
}

pub(crate) fn make_track_buttons() -> Vec<Button> {
    TRACKS
        .into_iter()
        .enumerate()
        .map(|(i, (text, color))| {
            let button_rect = Rect {
                x: TABLE_MARGIN.x + LEGEND_SPACING * i as f32 + LEGEND_SPACING * 0.5
                    - BUTTON_SIZE.x * 0.5,
                y: TABLE_MARGIN.y + TABLE_SIZE.y + vh(5.0),
                w: BUTTON_SIZE.x,
                h: BUTTON_SIZE.y,
            };
            Button::new(text, color, button_rect)
        })
        .collect()
}

pub(crate) fn update_all(ctx: &mut Context, buttons: &[Button]) {
    update_drag_state(&mut ctx.drag_state);
    for button in buttons {
        button.update(ctx);
    }
    update_tracks(ctx);
}

pub(crate) struct Context {
    screen_size: Vec2,
    drag_state: Option<DragState>,

    current_track: Track,
    fields: Vec<Option<Track>>,
}
