use grid::Grid;

pub type Node = isize;

#[derive(Debug, Clone)]
pub struct Nodes {
    grid: Grid<Node>,
}

impl Nodes {
    pub fn new(height: usize, width: usize) -> Self {
        let grid = Grid::new(height, width);

        Self { grid }
    }
}

impl std::ops::Deref for Nodes {
    type Target = Grid<Node>;

    fn deref(&self) -> &Self::Target {
        &self.grid
    }
}

impl std::ops::DerefMut for Nodes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.grid
    }
}
