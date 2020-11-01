mod app;
mod scene;
mod nodes;

pub use scene::*;
pub use nodes::Nodes;

pub mod consts {
    pub const WINDOW_HEIGHT: usize = 400;
    pub const WINDOW_WIDTH: usize = 400;
    pub const SCENE_TAIL_SIZE: usize = 16;
    pub const GRID_SIZE: usize = 2;
}

fn main() {
    app::run_app()
}
