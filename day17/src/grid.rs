//! Provides an expandable 3D grid.

use std::fmt;

/// An expandable 3D grid.
pub struct Expand3DGrid {
    grid: Vec<Vec<Vec<bool>>>,
    /// Width, height, and depth.
    size: (usize, usize, usize),
}

enum Axis {
    X,
    Y,
    Z,
}

/// A 3D grid that can expand in all directions.
impl Expand3DGrid {
    /// Checks if there are any cubes on a given edge.
    fn edge_has_cubes(&self, axis: Axis, positive: bool) -> bool {
        let (w, h, d) = self.size;
        let (mut x1, mut x2) = (0, w);
        let (mut y1, mut y2) = (0, h);
        let (mut z1, mut z2) = (0, d);
        match axis {
            Axis::X => {
                if positive {
                    x1 = w - 1;
                } else {
                    x1 = 0;
                }
                x2 = x1 + 1;
            }
            Axis::Y => {
                if positive {
                    y1 = h - 1;
                } else {
                    y1 = 0;
                }
                y2 = y1 + 1;
            }
            Axis::Z => {
                if positive {
                    z1 = d - 1;
                } else {
                    z1 = 0;
                }
                z2 = z1 + 1;
            }
        }
        for x in x1..x2 {
            for y in y1..y2 {
                for z in z1..z2 {
                    if self.has_cube((x, y, z)) {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Create a new grid with a starting width, height, and depth of `size`.
    pub fn new(size: (usize, usize, usize)) -> Self {
        let (w, h, d) = size;
        let mut grid = Vec::new();
        for _ in 0..d {
            let mut plane = Vec::new();
            for _ in 0..h {
                let mut row = Vec::new();
                for _ in 0..w {
                    row.push(false);
                }
                plane.push(row);
            }
            grid.push(plane);
        }
        Self {
            grid: grid,
            size: size,
        }
    }

    /// Toggles whether a position has a cube or not.
    pub fn toggle(&mut self, xyz: (usize, usize, usize)) {
        let (w, h, d) = self.size;
        let (x, y, z) = xyz;
        assert!(x < w);
        assert!(y < h);
        assert!(z < d);
        self.grid[z][y][x] = !self.grid[z][y][x];
    }

    /// Returns whether a position contains a cube or not.
    pub fn has_cube(&self, xyz: (usize, usize, usize)) -> bool {
        let (w, h, d) = self.size;
        let (x, y, z) = xyz;
        assert!(x < w);
        assert!(y < h);
        assert!(z < d);
        self.grid[z][y][x]
    }

    /// Counts the number of cubes adjacent to a cube.
    pub fn count_adj(&self, xyz: (usize, usize, usize)) -> usize {
        let (w, h, d) = self.size;
        let (x, y, z) = xyz;
        assert!(x < w);
        assert!(y < h);
        assert!(z < d);
        let x1 = if x == 0 { 0 } else { x - 1 };
        let x2 = if x == w - 1 { w } else { x + 2 };
        let y1 = if y == 0 { 0 } else { y - 1 };
        let y2 = if y == h - 1 { h } else { y + 2 };
        let z1 = if z == 0 { 0 } else { z - 1 };
        let z2 = if z == d - 1 { d } else { z + 2 };

        let mut cube_cnt = 0;
        for check_x in x1..x2 {
            for check_y in y1..y2 {
                for check_z in z1..z2 {
                    if x == check_x && y == check_y && z == check_z {
                        continue;
                    }
                    if self.has_cube((check_x, check_y, check_z)) {
                        cube_cnt += 1;
                    }
                }
            }
        }
        cube_cnt
    }

    /// Returns the (width, depth, height) of the grid.
    pub fn get_size(&mut self) -> (usize, usize, usize) {
        self.size
    }

    /// Expands the edges outward such that no voxels on any edge contains a cube.
    pub fn expand_border(&mut self) {
        // Find which edges, if any, need expansion.
        let x_far = self.edge_has_cubes(Axis::X, true);
        let x_near = self.edge_has_cubes(Axis::X, false);
        let y_far = self.edge_has_cubes(Axis::Y, true);
        let y_near = self.edge_has_cubes(Axis::Y, false);
        let z_far = self.edge_has_cubes(Axis::Z, true);
        let z_near = self.edge_has_cubes(Axis::Z, false);

        // Expand all edges that need expansion.
        let (mut w, mut h, mut d) = self.size;
        if x_far {
            for plane in self.grid.iter_mut() {
                for row in plane.iter_mut() {
                    row.push(false);
                }
            }
            w += 1
        }
        if x_near {
            for plane in self.grid.iter_mut() {
                for row in plane.iter_mut() {
                    row.insert(0, false);
                }
            }
            w += 1
        }
        if y_far {
            for plane in self.grid.iter_mut() {
                let mut new_row = Vec::new();
                for _ in 0..w {
                    new_row.push(false);
                }
                plane.push(new_row);
            }
            h += 1;
        }
        if y_near {
            for plane in self.grid.iter_mut() {
                let mut new_row = Vec::new();
                for _ in 0..w {
                    new_row.push(false);
                }
                plane.insert(0, new_row);
            }
            h += 1;
        }
        if z_far {
            let mut new_plane = Vec::new();
            for _ in 0..h {
                let mut row = Vec::new();
                for _ in 0..w {
                    row.push(false);
                }
                new_plane.push(row);
            }
            self.grid.push(new_plane);
            d += 1
        }
        if z_near {
            let mut new_plane = Vec::new();
            for _ in 0..h {
                let mut row = Vec::new();
                for _ in 0..w {
                    row.push(false);
                }
                new_plane.push(row);
            }
            self.grid.insert(0, new_plane);
            d += 1
        }
        self.size = (w, h, d);
    }

    /// Counts how many cubes the grid has.
    pub fn count_cubes(&mut self) -> usize {
        let mut cube_cnt = 0;
        for plane in self.grid.iter() {
            for row in plane.iter() {
                for has_cube in row.iter() {
                    if *has_cube {
                        cube_cnt += 1;
                    }
                }
            }
        }
        cube_cnt
    }
}

impl fmt::Display for Expand3DGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut plane_strs: Vec<String> = Vec::new();
        for (z, plane) in self.grid.iter().enumerate() {
            let mut lines: Vec<String> = Vec::new();
            lines.push(format!("z={}", z));
            for row in plane.iter() {
                lines.push(row.iter().map(|v| if *v { '#' } else { '.' }).collect())
            }
            plane_strs.push(lines.join("\n"));
        }
        write!(f, "{}", plane_strs.join("\n\n"))
    }
}
