use bevy::prelude::*;
use crate::Scene;
use crate::consts::*;

pub (super) fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    scene: Res<Scene>,
) {
    commands.spawn(Camera2dComponents::default());

    let tail_color = Color::rgb(0.9, 0.9, 0.9);
    let tail_size = SCENE_TAIL_SIZE as f32;

    for (node, position) in scene.iter_nodes() {
        commands.spawn(SpriteComponents {
            material: materials.add(tail_color.clone().into()),
            transform: Transform::from_translation(position.extend(0.0)),
            sprite: Sprite::new(Vec2::new(tail_size, tail_size)),
            ..Default::default()
        })
        .with(node);
    }
}
