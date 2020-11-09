use bevy::prelude::*;
use crate::consts::*;
use crate::Scene;

mod setup;
mod mouse;
mod update;

pub fn run_app() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "BIG bang".to_string(),
            width: WINDOW_WIDTH as u32,
            height: WINDOW_HEIGHT as u32,
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_resource(mouse::MousePosition::default())
        .add_resource(Scene::new())
        .add_default_plugins()
        .add_startup_system(setup::setup.system())
        .add_system(mouse::mouse_position.system())
        .add_system(mouse::mouse_click.system())
        .add_system(update::update.system())
        .run();
}
