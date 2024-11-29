use crate::{prelude::*, COLUMN_WIDTH, HOUR_HEIGHT, N_DAYS};

#[derive(Clone, Copy, Debug)]
pub(crate) struct Track {
    pub name: &'static str,
    pub clr: Color,
}

impl From<(&'static str, Color)> for Track {
    fn from(value: (&'static str, Color)) -> Self {
        Self {
            name: value.0,
            clr: value.1,
        }
    }
}

pub(crate) fn update_tracks(ctx: &mut Context) {
    if let Some(DragState::Dragging { start, current }) = ctx.drag_state {
        for i in get_field_at(start).unwrap()..=get_field_at(current).unwrap() {
            ctx.fields[i] = Some(ctx.current_track);
        }
    }
}

pub(crate) fn draw_tracks(fields: &Vec<Option<Track>>) {
    for (field, track) in fields.iter().enumerate() {
        if let Some(Track { clr, .. }) = track {
            draw_rect(get_field_rect(field), *clr);
        }
    }
}

pub(crate) fn get_field_rect(field: usize) -> Rect {
    let col = field / N_HOURS;
    let row = field % N_HOURS;
    Rect {
        x: TABLE_MARGIN.x + COLUMN_WIDTH * col as f32,
        y: TABLE_MARGIN.y + HOUR_HEIGHT * row as f32,
        w: COLUMN_WIDTH,
        h: HOUR_HEIGHT,
    }
}

pub(crate) fn get_field_at(pos: Vec2) -> Option<usize> {
    let pos = pos - TABLE_MARGIN;
    if pos != pos.abs() {
        return None;
    }
    let col = (pos.x / COLUMN_WIDTH) as usize;
    let row = (pos.y / HOUR_HEIGHT) as usize;
    let value = col * N_HOURS + row;
    if (0..(N_HOURS * N_DAYS)).contains(&value) {
        Some(value)
    } else {
        None
    }
}
