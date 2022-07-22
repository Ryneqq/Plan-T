use crate::{consts::*, Node, Point, Scene};
use bevy::{prelude::*, window::CursorMoved};
use itertools::Itertools;

#[derive(Debug, Default)]
pub struct MousePosition {
    pub position: Vec2,
}

pub(super) fn mouse_position(
    mut mouse: ResMut<MousePosition>,
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
    mut node_query: Query<(&Point, &mut Sprite)>,
) {
    let left_pressed = mouse_button.just_pressed(MouseButton::Left);
    let right_pressed = !left_pressed && mouse_button.just_pressed(MouseButton::Right);

    if left_pressed || right_pressed {
        let mouse_pos = mouse_position.position;
        let mouse_pos = Vec2::new(
            mouse_pos.x - WINDOW_WIDTH as f32 / 2.,
            mouse_pos.y - WINDOW_HEIGHT as f32 / 2.,
        );

        node_query
            .iter_mut()
            .sorted_by(|(x, _), (y, _)| scene.get_value(**y).cmp(&scene.get_value(**x)))
            .for_each(|(node, mut sprite)| {
                let node_pos = scene.point_to_world(node).unwrap();

                if distance(mouse_pos, node_pos) < SCENE_TAIL_SIZE as f32 / 2. {
                    if left_pressed {
                        scene.set_value(*node, Node::Alive);
                        sprite.color = Color::rgb(0.1, 1.0, 0.1);
                    } else if right_pressed {
                        scene.set_value(*node, Node::Dead);
                        sprite.color = Color::rgb(0.9, 0.9, 0.9);
                    }
                }
            });
    }
}

fn distance(one: Vec2, other: Vec2) -> f32 {
    ((one.x - other.x).powi(2) + (one.y - other.y).powi(2)).sqrt()
}
