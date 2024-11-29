use crate::{prelude::*, tracks::get_field_at};

pub(crate) fn update_drag_state(drag_state: &mut Option<DragState>) {
    *drag_state = match drag_state {
        None => {
            if is_mouse_button_pressed(MouseButton::Left) && get_field_at(mouse_pos()).is_some() {
                Some(DragState::Started(mouse_pos()))
            } else {
                None
            }
        }

        &mut Some(DragState::Started(start)) => {
            if is_mouse_button_released(MouseButton::Left) {
                // Dragging abort
                None
            } else {
                Some(DragState::Dragging {
                    start,
                    current: mouse_pos(),
                })
            }
        }

        &mut Some(DragState::Dragging { start, .. }) => {
            if is_mouse_button_released(MouseButton::Left) {
                Some(DragState::JustReleased {
                    start,
                    end: mouse_pos(),
                })
            } else {
                Some(DragState::Dragging {
                    start,
                    current: mouse_pos(),
                })
            }
        }

        &mut Some(DragState::JustReleased { .. }) => None,
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum DragState {
    Started(Vec2),
    Dragging { start: Vec2, current: Vec2 },
    JustReleased { start: Vec2, end: Vec2 },
}
