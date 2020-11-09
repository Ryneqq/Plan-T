mod app;
mod scene;
mod nodes;
mod primes;

pub use scene::*;
pub use nodes::*;
pub use primes::*;

pub mod consts {
    pub const WINDOW_HEIGHT: usize = 800;
    pub const WINDOW_WIDTH: usize = 800;
    pub const SCENE_TAIL_SIZE: usize = 16;
    pub const GRID_SIZE: usize = 2;
    pub const INFINITY: isize = 999999;
    pub const MAX_ADD: isize = INFINITY / 10;
}

fn main() {
    app::run_app()
}
