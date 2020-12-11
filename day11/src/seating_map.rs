/// A single tile for a seating map.
#[derive(Clone, Copy, PartialEq)]
enum SeatingTile {
    Floor,
    EmptySeat,
    FilledSeat,
}

/// Defines where seats are and whether they are occupied.
#[derive(PartialEq)]
pub struct SeatingMap {
    grid: Vec<Vec<SeatingTile>>,
}

impl SeatingMap {
    /// Creates a seating map of only floors with width `w` and height `h`.
    pub fn new(w: usize, h: usize) -> Self {
        let mut grid = Vec::new();
        for _ in 0..h {
            let mut row = Vec::new();
            for _ in 0..w {
                row.push(SeatingTile::Floor);
            }
            grid.push(row);
        }
        Self { grid: grid }
    }

    /// Sets the tile at position (`x`, `y`).
    fn set_tile(&mut self, x: usize, y: usize, tile: SeatingTile) {
        if x >= self.width() || y >= self.height() {
            // Out of bounds. Do nothing.
            return;
        }
        self.grid[y][x] = tile;
    }

    /// Gets the tile at position (`x`, `y`).
    fn get_tile(&self, x: usize, y: usize) -> SeatingTile {
        self.grid[y][x]
    }

    /// Counts the number of adjacent seats that are occupied.
    fn count_adj_filled(&self, x: usize, y: usize) -> usize {
        let w = self.width();
        let h = self.height();
        if x >= w || y >= h {
            // Out of bounds. Do nothing.
            return 0;
        }
        let mut cnt = 0;
        for dx in 0..3 {
            for dy in 0..3 {
                if (dx == 1 && dy == 1)
                    || (dx == 0 && x == 0)
                    || (dx == 2 && x == w - 1)
                    || (dy == 0 && y == 0)
                    || (dy == 2 && y == h - 1)
                {
                    continue;
                }
                match self.get_tile(x + dx - 1, y + dy - 1) {
                    SeatingTile::FilledSeat => cnt += 1,
                    _ => {}
                }
            }
        }
        cnt
    }

    /// Gets the width of the seating map.
    pub fn width(&self) -> usize {
        if self.grid.len() == 0 {
            0
        } else {
            self.grid[0].len()
        }
    }

    /// Gets the height of the seating map.
    pub fn height(&self) -> usize {
        self.grid.len()
    }

    /// Adds an empty seat at position (`x`, `y`).
    pub fn add_seat(&mut self, x: usize, y: usize) {
        self.set_tile(x, y, SeatingTile::EmptySeat);
    }

    /// Counts the number of seats that are occupied.
    pub fn count_filled_seats(&self) -> usize {
        let mut cnt = 0;
        for row in self.grid.iter() {
            for tile in row.iter() {
                match tile {
                    SeatingTile::FilledSeat => cnt += 1,
                    _ => {}
                }
            }
        }
        cnt
    }

    /// Moves seating occupants around. May not necessarily change anything.
    pub fn next(&self) -> Self {
        let mut next_map = self.clone();
        for y in 0..self.height() {
            for x in 0..self.width() {
                match self.get_tile(x, y) {
                    SeatingTile::FilledSeat => {
                        if self.count_adj_filled(x, y) >= 4 {
                            next_map.set_tile(x, y, SeatingTile::EmptySeat)
                        }
                    }
                    SeatingTile::EmptySeat => {
                        if self.count_adj_filled(x, y) == 0 {
                            next_map.set_tile(x, y, SeatingTile::FilledSeat)
                        }
                    }
                    _ => {}
                }
            }
        }
        next_map
    }

    /// Shuffles seat occupants around until seat positions no longer change.
    pub fn get_stable(&self) -> Self {
        let mut prev = self.next();
        loop {
            let next = prev.next();
            if next == prev {
                return prev;
            }
            prev = next;
        }
    }
}

impl Clone for SeatingMap {
    fn clone(&self) -> Self {
        let mut grid = Vec::new();
        for row in self.grid.iter() {
            grid.push(row.clone());
        }
        Self { grid: grid }
    }
}
