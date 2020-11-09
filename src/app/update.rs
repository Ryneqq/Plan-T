use crate::Point;
use crate::Scene;
use crate::consts::*;
use bevy::prelude::*;
use itertools::Itertools;

pub fn update(
    mut scene: ResMut<Scene>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    node_query: Query<(&Point, &Handle<ColorMaterial>)>
) {
    node_query.iter()
        .sorted_by(|(x, _), (y, _)| scene.get_value(**y).cmp(&scene.get_value(**x)))
        .for_each(|(node, material)| {
            let mut material = materials.get_mut(material).unwrap();
            let color_divider = (MAX_ADD / 10) as f32;
            scene.update(*node);


            if scene.is_stable(*node) {
                let value = scene.get_value(*node);
                let green = value as f32 / color_divider;

                material.color = Color::rgb(1.0 - green, 1.0, 1.0 - green);
            } else {
                let red = scene.get_value(*node) as f32 / color_divider;

                material.color = Color::rgb(1.0, 1.0 - red, 1.0 - red);
            }
        });
}
