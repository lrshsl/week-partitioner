use draw_functions::{draw_all, draw_fps, draw_screen};
use prelude::*;

mod constants;
mod draw_functions;
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
    let render_target = render_target(RESOLUTION.x as u32, RESOLUTION.y as u32);

    let screen_buffer = Camera2D {
        zoom: Vec2::ONE / AREA_SIZE * 2.0,
        offset: Vec2::NEG_ONE,
        render_target: Some(render_target),
        ..Default::default()
    };

    let mut ctx = Context {
        screen_size: vec2(0.0, 0.0),
    };

    loop {
        // Draw to texture
        if let Some(_) = get_action(&mut ctx) {
            set_camera(&screen_buffer);
            draw_all();
        }
        draw_fps();

        // Draw texture to screen
        set_default_camera();
        draw_screen(&screen_buffer.render_target.as_ref().unwrap().texture);

        next_frame().await
    }
}

pub(crate) enum Action {
    Click(Vec2),
    WindowResize(Vec2),
}

struct Context {
    screen_size: Vec2,
}

pub(crate) fn get_action(ctx: &mut Context) -> Option<Action> {
    if is_mouse_button_pressed(MouseButton::Left) {
        return Some(Action::Click(mouse_position().into()));
    }

    let size = screen_size().into();
    if size != ctx.screen_size.into() {
        ctx.screen_size = size;
        return Some(Action::WindowResize(size));
    }

    None
}
