use crate::{consts::*, Point, Scene};
use bevy::{prelude::*, window::CursorMoved};
use std::cmp::min;

// #[derive(Default)]
// pub struct MouseState {
//     cursor_moved_event_reader: EventReader<CursorMoved>,
// }

#[derive(Debug, Default)]
pub struct MousePosition {
    pub position: Vec2,
}

pub(super) fn mouse_position(
    mut mouse: ResMut<MousePosition>,
    // mut state: Local<MouseState>,
    mut cursor_moved_events: EventReader<CursorMoved>,
) {
    for event in cursor_moved_events.iter() {
        mouse.position = event.position;
    }
}

pub(super) fn mouse_click(
    mouse_button: Res<Input<MouseButton>>,
    mouse_position: Res<MousePosition>,
    mut scene: ResMut<Scene>,
    node_query: Query<&Point>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        let mouse_pos = mouse_position.position;
        let mouse_pos = Vec2::new(
            mouse_pos.x - WINDOW_WIDTH as f32 / 2.,
            mouse_pos.y - WINDOW_HEIGHT as f32 / 2.,
        );

        node_query
            .iter()
            .find(|node| {
                let node_pos = scene.point_to_world(node).unwrap();

                distance(mouse_pos, node_pos) < SCENE_TAIL_SIZE as f32 / 2.
            })
            .map(|node| {
                let current = scene.get_value(*node);

                scene.set_value(*node, min(INFINITY, current + MAX_ADD));
            });
    }
}

fn distance(one: Vec2, other: Vec2) -> f32 {
    ((one.x - other.x).powi(2) + (one.y - other.y).powi(2)).sqrt()
}
