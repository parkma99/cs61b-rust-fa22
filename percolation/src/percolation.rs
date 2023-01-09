use crate::wqu::WeightedQuickUnion;

pub struct Percolation {
    grid: Vec<bool>,
    wqu: WeightedQuickUnion,
    width: usize,
    height: usize,
    nums_of_open: usize,
}

pub trait Percolatable {
    fn new(width: usize, height: usize) -> Self;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn open(&mut self, row: usize, col: usize);
    fn is_open(&self, row: usize, col: usize) -> bool;
    fn is_full(&self, row: usize, col: usize) -> bool;
    fn number_of_open_sites(&self) -> usize;
    fn percolates(&self) -> bool;
}

impl Percolation {
    fn index(&self, row: usize, col: usize) -> usize {
        row * self.width + col + 1
    }
    fn is_valid(&self, row: usize, col: usize) -> bool {
        row < self.height && col < self.width
    }
}

impl Percolatable for Percolation {
    fn new(width: usize, height: usize) -> Self {
        let size = width * height + 2;
        let mut grid = Vec::new();
        grid.resize(size, false);
        grid[0] = true;
        grid[size - 1] = true;
        let wqu = WeightedQuickUnion::new(size);
        Self {
            grid,
            wqu,
            width,
            height,
            nums_of_open: 0,
        }
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn open(&mut self, row: usize, col: usize) {
        if self.is_valid(row, col) && !self.is_open(row, col) {
            let index = self.index(row, col);
            self.grid[index] = true;
            self.nums_of_open += 1;
            let mut neighbors = Vec::new();
            if row > 0 && self.is_open(row - 1, col) {
                neighbors.push(self.index(row - 1, col));
            }
            if row < self.height - 1 && self.is_open(row + 1, col) {
                neighbors.push(self.index(row + 1, col));
            }
            if col > 0 && self.is_open(row, col - 1) {
                neighbors.push(self.index(row, col - 1));
            }
            if col < self.width - 1 && self.is_open(row, col + 1) {
                neighbors.push(self.index(row, col + 1));
            }
            for neighbor in neighbors {
                self.wqu.union(index, neighbor);
            }
            if row == 0 {
                self.wqu.union(0, index);
            }
            if row == self.height - 1 {
                self.wqu.union(self.grid.len() - 1, index);
            }
        }
    }

    fn is_open(&self, row: usize, col: usize) -> bool {
        match self.is_valid(row, col) {
            true => {
                let index = self.index(row, col);
                self.grid[index]
            }
            false => false,
        }
    }

    fn is_full(&self, row: usize, col: usize) -> bool {
        match self.is_valid(row, col) {
            true => {
                let index = self.index(row, col);
                self.wqu.connected(0, index)
            }
            false => false,
        }
    }

    fn number_of_open_sites(&self) -> usize {
        self.nums_of_open
    }

    fn percolates(&self) -> bool {
        self.wqu.connected(0, self.grid.len() - 1)
    }
}
