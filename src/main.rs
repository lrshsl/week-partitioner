use draw_functions::{draw_all, draw_fps, draw_screen};
use prelude::*;

mod button;
mod constants;
mod draw_functions;
mod prelude;
mod table_context;
mod time;
mod util;

pub(crate) const DAY_START: Time = hours(6);
pub(crate) const DAY_END: Time = hours(21);

pub(crate) const N_TRACKS: usize = 3;
pub(crate) const TRACKS: [(&'static str, Color); N_TRACKS] =
    [("Analysis II", RED), ("Algs", GREEN), ("LinAlg", YELLOW)];
pub(crate) const LEGEND_SPACING: f32 = TABLE_SIZE.x / N_TRACKS as f32;

pub(crate) const COLUMN_WIDTH: f32 = TABLE_SIZE.x / 7.0;

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
        current_track: Track::from(TRACKS[0]),
    };

    let track_buttons = make_track_buttons();

    loop {
        if is_mouse_button_pressed(MouseButton::Middle) {
            println!(
                "Screen pos {:?} -> buffer pos {:?}",
                mouse_position(),
                to_buffer_cords(mouse_position().into()),
            );
        }

        // Draw to texture
        update_all(&mut ctx, &track_buttons);
        set_camera(&screen_buffer);
        draw_all(&track_buttons);
        draw_fps();

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
    for button in buttons {
        button.update(ctx);
    }
}

pub(crate) enum Action {
    Click(Vec2),
    WindowResize(Vec2),
    StartDrag(Vec2),
    Dragging { start: Vec2, current: Vec2 },
    EndDrag { start: Vec2, end: Vec2 },
}

pub(crate) struct Track {
    name: &'static str,
    clr: Color,
}

impl From<(&'static str, Color)> for Track {
    fn from(value: (&'static str, Color)) -> Self {
        Self {
            name: value.0,
            clr: value.1,
        }
    }
}

pub(crate) struct Context {
    screen_size: Vec2,
    current_track: Track,
}
