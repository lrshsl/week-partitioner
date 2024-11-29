use crate::{prelude::*, COLUMN_WIDTH, HOUR_HEIGHT, N_DAYS};

#[derive(Clone, Copy, Debug)]
pub(crate) struct TrackData {
    pub name: &'static str,
    pub clr: Color,
}
impl TrackData {
    pub(crate) const fn new(name: &'static str, clr: Color) -> Self {
        Self { name, clr }
    }
}

impl From<(&'static str, Color)> for TrackData {
    fn from(value: (&'static str, Color)) -> Self {
        Self {
            name: value.0,
            clr: value.1,
        }
    }
}

pub(crate) fn update_tracks(ctx: &mut Context) {
    if let Some(DragState::Dragging { start, current }) = ctx.drag_state {
        let f1 = get_field_at(start).unwrap();
        let f2 = get_field_at(current).unwrap();
        for i in f1.min(f2)..=f1.max(f2) {
            ctx.fields[i] = Some(ctx.current_track);
        }
    }
}

pub(crate) fn draw_tracks(ctx: &Context) {
    for (field, track) in ctx.fields.iter().enumerate() {
        if let Some(track_id) = track {
            draw_rect(get_field_rect(field), ctx.track_list[*track_id].clr);
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
    if !(0..N_DAYS).contains(&col) || !(0..N_HOURS).contains(&row) {
        return None;
    }

    let value = col * N_HOURS + row;
    if (0..(N_HOURS * N_DAYS)).contains(&value) {
        Some(value)
    } else {
        None
    }
}
