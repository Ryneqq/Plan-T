use crate::consts::*;
use crate::Scene;
use bevy::prelude::*;

pub(super) fn setup(mut commands: Commands, scene: Res<Scene>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let tail_color = Color::rgb(0.9, 0.9, 0.9);
    let tail_size = SCENE_TAIL_SIZE as f32;

    for (node, position) in scene.iter_world() {
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform::from_translation(position.extend(0.0)),
                sprite: Sprite {
                    color: tail_color,
                    custom_size: Some(Vec2::new(tail_size, tail_size)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(node);
    }
}
