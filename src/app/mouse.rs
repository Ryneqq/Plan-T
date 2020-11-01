use crate:: {Scene, Point, consts::*};
use bevy::{
    prelude::*,
    window::CursorMoved,
};

#[derive(Default)]
pub struct MouseState {
    cursor_moved_event_reader: EventReader<CursorMoved>,
}

#[derive(Debug, Default)]
pub struct MousePosition {
    pub position: Vec2
}

pub (super) fn mouse_position(
    mut mouse: ResMut<MousePosition>,
    mut state: Local<MouseState>,
    cursor_moved_events: Res<Events<CursorMoved>>,
) {
    for event in state.cursor_moved_event_reader.iter(&cursor_moved_events) {
        mouse.position = event.position;
    }
}

pub (super) fn mouse_click(
    mouse_button: Res<Input<MouseButton>>,
    mouse_position: Res<MousePosition>,
    scene: Res<Scene>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    node_query: Query<(&Point, &Handle<ColorMaterial>)>
) {
    if mouse_button.pressed(MouseButton::Left) {
        let mouse_pos = mouse_position.position;
        let mouse_pos = Vec2::new(
            mouse_pos.x() - WINDOW_WIDTH as f32 / 2.,
            mouse_pos.y() - WINDOW_HEIGHT as f32 / 2.,
        );

        node_query.iter()
            .find(|(node, _material)| {
                let node_pos = scene.point_to_world(node).unwrap();

                distance(mouse_pos, node_pos) < SCENE_TAIL_SIZE as f32 / 2.
            })
            .map(|(_node, material)| {
                let mut material = materials.get_mut(material).unwrap();

                material.color = Color::rgb(1.0, 0.0, 0.0);
            });
    }
}

fn distance(one: Vec2, other: Vec2) -> f32 {
    ((one.x() - other.x()).powi(2) + (one.y() - other.y()).powi(2)).sqrt()
}
