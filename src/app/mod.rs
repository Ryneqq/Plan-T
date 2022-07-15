use crate::consts::*;
use crate::Scene;
use bevy::prelude::*;

mod mouse;
mod setup;
mod update;

pub fn run_app() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Plan-T".to_string(),
            width: WINDOW_WIDTH as f32,
            height: WINDOW_HEIGHT as f32,
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(mouse::MousePosition::default())
        .insert_resource(update::Pause::default())
        .insert_resource(Scene::new())
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup::setup)
        .add_system(mouse::mouse_position)
        .add_system(mouse::mouse_click)
        .add_system(update::update)
        .run();
}
