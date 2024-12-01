use crate::{prelude::*, TrackId, COLUMN_WIDTH, HOUR_HEIGHT, N_DAYS};

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
    if let Some(current_track) = ctx.current_track {
        match ctx.drag_state {
            Some(DragState::Dragging { start, current }) => {
                let Some(f1) = get_field_at(start) else {
                    return;
                };
                let Some(f2) = get_field_at(current) else {
                    return;
                };
                ctx.tmp_fields.iter_mut().for_each(|x| x.clear());
                ctx.selection.clear();
                for i in f1.min(f2)..=f1.max(f2) {
                    ctx.tmp_fields[i].insert(current_track);
                }
            }
            Some(DragState::JustReleased { .. }) => {
                for (field, tmp_field) in ctx.fields.iter_mut().zip(ctx.tmp_fields.iter()) {
                    field.extend(tmp_field);
                }
            }
            _ => {}
        }
    }
}

pub(crate) fn draw_tracks(ctx: &Context) {
    for (field_i, (tracks, tmp_tracks)) in ctx.fields.iter().zip(ctx.tmp_fields.iter()).enumerate()
    {
        let all_tracks = tracks.union(tmp_tracks);
        for &track_id in all_tracks {
            let mut r = get_field_rect(field_i);
            r.w /= N_TRACKS as f32;
            r.x += r.w * track_id as f32;
            draw_rect(r, ctx.track_list[track_id].clr);
            if ctx.selection.contains(&(field_i, track_id)) {
                draw_rect_lines(r, THICK_LINES, WHITE);
            }
        }
    }
}

pub(crate) fn get_track_at(pos: Vec2) -> Option<(usize, TrackId)> {
    let field = get_field_at(pos)?;
    let pos = pos - TABLE_MARGIN;
    let x_in_col = pos.x % COLUMN_WIDTH;
    let track = (x_in_col / (COLUMN_WIDTH / N_TRACKS as f32)) as usize;
    Some((field, track))
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
