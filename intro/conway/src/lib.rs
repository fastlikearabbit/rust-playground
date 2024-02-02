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
            grid: vec![Default::default(); rows * cols],
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

        for i in row.saturating_sub(1)..=(row + 1).min(self.rows - 1) {
            for j in col.saturating_sub(1)..=(col + 1).min(self.cols - 1) {
               if i != row || j != col {
                    neighbours.push((i, j));
               }
            }
        }
        neighbours
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
        Self {
            grid
        }
    }

    pub fn get_grid(&self) -> &Grid<Cell> {
        &self.grid
    }

    pub fn step(&mut self) {
        let (rows, cols) = self.grid.size();
        let mut new_grid = Grid::new(rows, cols);

        for i in 0..rows {
            for j in 0..cols {
                let alive_neighbours_count = self.get_alive_neighbours_count(i, j);
                match self.grid.get(i, j) {
                    Cell::Alive => match alive_neighbours_count {
                        0 | 1  => new_grid.set(Cell::Dead, i, j),
                        2 | 3  => new_grid.set(Cell::Alive, i, j),
                        _      => new_grid.set(Cell::Dead, i, j),
                    },
                    Cell::Dead => if alive_neighbours_count == 3 {
                        new_grid.set(Cell::Alive, i, j);
                    }
                }
            }
        }
        self.grid = new_grid;
    }

    fn get_alive_neighbours_count(&self, row: usize, col: usize) -> usize {
        let mut count = 0;
        for (i, j) in self.grid.neighbours(row, col) {
            if let Cell::Alive = self.grid.get(i, j) {
                count += 1;
            }
        }
        count
    }
}
