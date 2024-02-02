#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq, Eq)]
pub struct Grid<T> {
    rows: usize,
    cols: usize,
    grid: Vec<T>,
}

impl<T: Clone + Default> Grid<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            grid: Vec::with_capacity(rows * cols),
        }
    }

    pub fn from_slice(grid: &[T], rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            grid: Vec::from(grid),
        }

    }

    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.grid[row * self.cols + col]
    }

    pub fn set(&mut self, value: T, row: usize, col: usize) {
        self.grid[row * self.cols + col] = value;
    }

    pub fn neighbours(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut neighbours = Vec::new();

        for i in row.saturating_sub(1)..=(row + 1).saturating_sub(self.rows - 1) {
            for j in col.saturating_sub(1)..=(col + 1).saturating_sub(self.cols - 1) {
               if i != row || j != col {
                    neighbours.push((i, j));
               }
            }
        }
        neighbours
    }

    fn in_bounds(&self, row: usize, col: usize) -> bool {
        row >= 0 && row < self.rows && col >= 0 && col < self.cols   
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Dead,
    Alive,
}

impl Default for Cell {
    fn default() -> Self {
        Self::Dead
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq)]
pub struct GameOfLife {
    grid: Grid<Cell>,
}

impl GameOfLife {
    pub fn from_grid(grid: Grid<Cell>) -> Self {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn get_grid(&self) -> &Grid<Cell> {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn step(&mut self) {
        // TODO: your code goes here.
        unimplemented!()
    }
}
