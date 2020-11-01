use bevy::prelude::Vec2;
use std::collections::HashMap;
use crate::Nodes;
use crate::consts::*;

pub type Point = (usize, usize);

#[derive(Debug, Clone)]
pub struct Scene {
    world: HashMap<Point, Vec2>,
    nodes: Nodes,
}

impl Scene {
    pub fn new() -> Self {
        let grid_height = WINDOW_HEIGHT / (SCENE_TAIL_SIZE + 2 * GRID_SIZE);
        let grid_width = WINDOW_WIDTH / (SCENE_TAIL_SIZE + 2 * GRID_SIZE);
        let nodes = Nodes::new(grid_height, grid_width);
        let top_left_corner = (
            -((WINDOW_WIDTH / 2) as isize) + (SCENE_TAIL_SIZE / 2 + GRID_SIZE) as isize,
            (WINDOW_HEIGHT / 2) as isize - ((SCENE_TAIL_SIZE / 2 + GRID_SIZE) as isize),
        );
        let (x0, y0) = top_left_corner;
        let mut new = Self { world: Default::default(), nodes };

        for i in 0..grid_width {
            for j in 0..grid_height {
                let x = x0 + (i * (SCENE_TAIL_SIZE + 2 * GRID_SIZE)) as isize;
                let y = y0 - (j * (SCENE_TAIL_SIZE + 2 * GRID_SIZE)) as isize;

                new.insert((i, j), Vec2::new(x as f32, y as f32));
            }
        }

        new
    }

    pub fn iter_nodes<'a>(&'a self) -> impl Iterator<Item = (Point, Vec2)> + 'a {
        self.world.iter().map(|(point, position)| (*point, *position))
    }

    pub fn set_value(&mut self, (x, y): Point, value: usize) {
        self.nodes[x][y] = value;
    }

    pub fn point_to_world(&self, index: &Point) -> Option<Vec2> {
        self.world.get(index).copied()
    }

    fn insert(&mut self, index: Point, world: Vec2) {
        self.world.insert(index, world);
    }
}
