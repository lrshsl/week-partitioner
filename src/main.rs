use std::collections::HashSet;

use crate::drag_state::update_drag_state;
use button::TrackButton;
use draw_functions::{draw_all, draw_fps, draw_screen};
use macroquad::{
    camera::{set_camera, set_default_camera, Camera2D},
    math::Rect,
    texture::render_target,
    window::next_frame,
};
use prelude::*;
use tracks::{get_track_at, update_tracks};

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
pub(crate) const TRACKS: [TrackData; N_TRACKS] = [
    TrackData::new("Analysis II", RED),
    TrackData::new("Algs", GREEN),
    TrackData::new("LinAlg", YELLOW),
];

pub(crate) const COLUMN_WIDTH: f32 = TABLE_SIZE.x / N_DAYS as f32;
pub(crate) const LEGEND_SPACING: f32 = TABLE_SIZE.x / N_TRACKS as f32;

pub(crate) const N_DAYS: usize = 7;
pub(crate) const DAY_NAMES: [&'static str; N_DAYS] =
    ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

#[macroquad::main("Week partitioner")]
async fn main() {
    let render_target = render_target(RESOLUTION.x as u32, RESOLUTION.y as u32);

    let screen_buffer = Camera2D {
        zoom: Vec2::ONE / AREA_SIZE * 2.0,
        offset: Vec2::NEG_ONE,
        render_target: Some(render_target),
        ..Default::default()
    };

    let mut ctx = Context {
        screen_size: vec2(0.0, 0.0),
        drag_state: None,
        selection: HashSet::new(),

        track_list: TRACKS.into(),
        current_track: None,

        fields: vec![HashSet::new(); N_DAYS * N_HOURS],
        tmp_fields: vec![HashSet::new(); N_DAYS * N_HOURS],
    };

    let track_buttons = make_track_buttons(&ctx);

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

pub(crate) fn make_track_buttons(ctx: &Context) -> Vec<TrackButton> {
    ctx.track_list
        .iter()
        .enumerate()
        .map(|(track_id, track)| {
            let button_rect = Rect {
                x: TABLE_MARGIN.x + LEGEND_SPACING * track_id as f32 + LEGEND_SPACING * 0.5
                    - BUTTON_SIZE.x * 0.5,
                y: TABLE_MARGIN.y + TABLE_SIZE.y + vh(5.0),
                w: BUTTON_SIZE.x,
                h: BUTTON_SIZE.y,
            };
            TrackButton::new(track_id, track.name, button_rect, track.clr, BLACK)
        })
        .collect()
}

pub(crate) fn update_all(ctx: &mut Context, buttons: &[TrackButton]) {
    update_drag_state(&mut ctx.drag_state);
    let any_button_clicked = buttons.iter().enumerate().any(|(i, track_button)| {
        if track_button.is_clicked() {
            if ctx.current_track == Some(track_button.id) {
                ctx.current_track = None;
            } else {
                ctx.current_track = Some(i);
            }
            true
        } else {
            false
        }
    });
    if !any_button_clicked
        && is_mouse_button_pressed(MouseButton::Left)
        && !TABLE_RECT.contains(mouse_pos())
    {
        ctx.current_track = None;
        ctx.selection.clear();
    }
    update_tracks(ctx);
    update_selection(ctx);
}

fn update_selection(ctx: &mut Context) {
    if ctx.current_track.is_none() {
        match ctx.drag_state {
            Some(DragState::Dragging { current, .. }) => {
                if let Some(track_selection) = get_track_at(current) {
                    ctx.selection.insert(track_selection);
                }
            }
            Some(DragState::JustReleased { start, end }) => {
                // Single click (still in the same field)
                let first_track = get_track_at(start);

                if first_track.is_some() && first_track == get_track_at(end) {
                    let track_selection = first_track.unwrap();
                    if ctx.selection.contains(&track_selection) {
                        ctx.selection.remove(&track_selection);
                    } else {
                        ctx.selection.insert(track_selection);
                    }
                } else {
                    // Deselect
                    ctx.current_track = None;
                }
            }
            _ => {}
        }
    }
}

pub(crate) struct Context {
    screen_size: Vec2,
    drag_state: Option<DragState>,
    selection: HashSet<(usize, TrackId)>,

    track_list: Vec<TrackData>,
    current_track: Option<TrackId>,

    fields: Vec<HashSet<TrackId>>,
    tmp_fields: Vec<HashSet<TrackId>>,
}

pub(crate) type TrackId = usize;
