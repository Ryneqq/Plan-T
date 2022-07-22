use crate::{Node, Point, Scene};
use bevy::prelude::*;

#[derive(Debug)]
pub struct Pause {
    pub paused: bool,
}

impl Pause {
    pub fn switch(&mut self) {
        self.paused = !self.paused;
    }
}

impl Default for Pause {
    fn default() -> Self {
        Self { paused: true }
    }
}

pub fn green() -> Color {
    Color::rgb(0.1, 0.6, 0.3)
}

pub fn red() -> Color {
    Color::rgb(0.6, 0.3, 0.1)
}

pub fn white() -> Color {
    Color::rgb(0.9, 0.9, 0.9)
}

pub fn gray() -> Color {
    Color::rgb(0.5, 0.5, 0.5)
}

pub fn update(
    input: Res<Input<KeyCode>>,
    mut pause: ResMut<Pause>,
    mut scene: ResMut<Scene>,
    mut node_query: Query<(&Point, &mut Sprite)>,
) {
    if input.just_pressed(KeyCode::Space) {
        pause.switch();
    }

    if !pause.paused || input.just_pressed(KeyCode::S) {
        scene.update();

        node_query
            .iter_mut()
            .for_each(|(node, mut sprite)| match scene.get_value(*node) {
                Node::Alive => sprite.color = green(),
                Node::Dead => sprite.color = white(),
                Node::WillBeDead => sprite.color = red(),
                Node::WillBeAlive => sprite.color = gray(),
            });
    }

    if input.just_pressed(KeyCode::C) {
        node_query.iter_mut().for_each(|(node, mut sprite)| {
            scene.set_value(*node, Node::Dead);

            match scene.get_value(*node) {
                _ => sprite.color = white(),
            }
        });
    }
}
