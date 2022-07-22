use crate::consts::*;
use crate::{Node, Nodes};
use bevy::prelude::{Component, Vec2};
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Component, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct Point(pub usize, pub usize);

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Point {
        Point(x, y)
    }
}

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
        let mut new = Self {
            world: Default::default(),
            nodes,
        };

        for i in 0..grid_width {
            for j in 0..grid_height {
                let x = x0 + (i * (SCENE_TAIL_SIZE + 2 * GRID_SIZE)) as isize;
                let y = y0 - (j * (SCENE_TAIL_SIZE + 2 * GRID_SIZE)) as isize;

                new.insert((i, j).into(), Vec2::new(x as f32, y as f32));
            }
        }

        new
    }

    pub fn iter_world<'a>(&'a self) -> impl Iterator<Item = (Point, Vec2)> + 'a {
        self.world
            .iter()
            .map(|(point, position)| (*point, *position))
    }

    pub fn iter_points<'a>(&'a self) -> impl Iterator<Item = Point> + 'a {
        self.world.iter().map(|(point, _)| *point)
    }

    pub fn set_value(&mut self, Point(x, y): Point, value: Node) {
        self.nodes[x][y] = value;
    }

    pub fn get_value(&self, Point(x, y): Point) -> Node {
        self.nodes[x][y]
    }

    pub fn point_to_world(&self, index: &Point) -> Option<Vec2> {
        self.world.get(index).copied()
    }

    pub fn iter_close_neighbors<'a>(
        &'a self,
        Point(x, y): Point,
    ) -> impl Iterator<Item = Point> + 'a {
        vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
            .into_iter()
            .flat_map(move |node| {
                let node = node.into();
                self.world.get(&node).map(|_| node)
            })
    }

    pub fn iter_further_neighbors<'a>(
        &'a self,
        Point(x, y): Point,
    ) -> impl Iterator<Item = Point> + 'a {
        vec![
            (x + 1, y + 1),
            (x - 1, y - 1),
            (x + 1, y - 1),
            (x - 1, y + 1),
        ]
        .into_iter()
        .flat_map(move |node| {
            let node = node.into();
            self.world.get(&node).map(|_| node)
        })
    }

    pub fn iter_neighbors<'a>(&'a self, point: Point) -> impl Iterator<Item = Point> + 'a {
        self.iter_close_neighbors(point)
            .chain(self.iter_further_neighbors(point))
    }

    pub fn update(&mut self) {
        let new_nodes = self
            .iter_points()
            .map(|point| {
                let mut neighbors = 0;
                for neighbor in self.iter_neighbors(point) {
                    if self.get_value(neighbor) == Node::Alive {
                        neighbors += 1;
                    }
                }

                let value = match self.get_value(point) {
                    Node::WillBeAlive => Node::Alive,
                    Node::WillBeDead => Node::Dead,
                    Node::Dead => {
                        if neighbors == 3 {
                            Node::WillBeAlive
                        } else {
                            Node::Dead
                        }
                    }
                    Node::Alive => {
                        if neighbors == 2 || neighbors == 3 {
                            // if neighbors <= 3 {
                            Node::Alive
                        } else {
                            Node::WillBeDead
                        }
                    }
                };

                (point, value)
            })
            .collect_vec();

        for (point, value) in new_nodes {
            self.set_value(point, value);
        }
    }

    // fn update_stable(&mut self, origin: Point) {
    //     let neighbours = self.iter_neighbors(origin).collect_vec();
    //     let current_value = self.get_value(origin);

    //     if current_value < 2 {
    //         return;
    //     };

    //     neighbours
    //         .iter()
    //         .sorted_by(|x, y| self.get_value(**x).cmp(&self.get_value(**y)))
    //         .next()
    //         .map(|point| {
    //             let swap_value = self.get_value(*point);

    //             // TODO: and swap point is not stable?
    //             if swap_value < current_value {
    //                 self.set_value(*point, current_value);
    //                 self.set_value(origin, swap_value);
    //             }
    //         });
    // }

    // fn update_not_stable(&mut self, point: Point) {
    //     let neighbours = self.iter_neighbors(point).collect_vec();
    //     let mut current_value = self.get_value(point);

    //     if current_value < 2 {
    //         return;
    //     };

    //     neighbours
    //         .iter()
    //         .sorted_by(|x, y| self.get_value(**x).cmp(&self.get_value(**y)))
    //         .for_each(|point| {
    //             if current_value < 2 {
    //                 return;
    //             };
    //             let neighbor_value = self.get_value(*point);
    //             let divisors = self.primes.divisors(current_value.abs() as usize);
    //             let spread_value = divisors
    //                 .map(|div| div as isize)
    //                 .find(|div| {
    //                     &current_value != div
    //                         && neighbor_value + current_value / div <= current_value
    //                 })
    //                 .map(|div| current_value / div);

    //             if let Some(spread_value) = spread_value {
    //                 current_value -= spread_value;
    //                 self.set_value(*point, neighbor_value + spread_value)
    //             }
    //         });

    //     self.set_value(point, current_value)
    // }

    // pub fn is_stable(&self, point: Point) -> bool {
    //     let current_value = self.get_value(point).abs() as usize;

    //     self.primes
    //         .nearest_prime(current_value)
    //         .map(|prime| current_value == prime)
    //         .unwrap_or(false)
    // }

    fn insert(&mut self, index: Point, world: Vec2) {
        self.world.insert(index, world);
    }
}
