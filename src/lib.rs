mod utils;

use core::fmt;

use wasm_bindgen::prelude::*;

extern crate web_sys;
use web_sys::console;

pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        console::time_with_label(name);
        Timer { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<bool>,
}

#[wasm_bindgen]
impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (self.width * row + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }

        count
    }

    pub fn tick(&mut self) {
        let _timer = Timer::new("Universe::tick");
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    (true, x) if x < 2 => false,
                    (true, 2 | 3) => true,
                    (true, x) if x > 3 => false,
                    (false, 3) => true,
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    pub fn new() -> Universe {
        let width = 64 * 5;
        let height = 64 * 2;

        let cells = (0..width * height).map(|_| false).collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn randomize(&mut self) {
        let new_cells = (0..self.width * self.height)
            .map(|_| js_sys::Math::random() < 0.5)
            .collect();

        self.cells = new_cells;
    }

    pub fn clear(&mut self) {
        let width = 64 * 5;
        let height = 64 * 2;

        let new_cells = (0..width * height).map(|_| false).collect();

        self.cells = new_cells;
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const bool {
        self.cells.as_ptr()
    }

    pub fn set_cell(&mut self, row: u32, column: u32, new_value: bool) {
        let idx = self.get_index(row, column);

        self.cells[idx] = new_value;
    }

    pub fn insert_glider(&mut self, row: u32, column: u32) {
        let glider_indexes = [
            self.get_index(row, column),
            self.get_index(row, column - 1),
            self.get_index(row, column + 1),
            self.get_index(row - 1, column + 1),
            self.get_index(row - 2, column),
        ];

        for idx in glider_indexes {
            self.cells[idx] = true;
        }
    }

    pub fn insert_pulsar(&mut self, row: u32, column: u32) {
        let pulsar_indexes = [
            // First pattern at top left
            self.get_index(row - 1, column - 2),
            self.get_index(row - 1, column - 3),
            self.get_index(row - 1, column - 4), // First line
            self.get_index(row - 2, column - 1),
            self.get_index(row - 3, column - 1),
            self.get_index(row - 4, column - 1), // Second line
            self.get_index(row - 2, column - 6),
            self.get_index(row - 3, column - 6),
            self.get_index(row - 4, column - 6), // Third line (most left one)
            self.get_index(row - 6, column - 2),
            self.get_index(row - 6, column - 3),
            self.get_index(row - 6, column - 4), // Fourth line (most top one)
            // Second pattern at top right
            self.get_index(row - 1, column + 2),
            self.get_index(row - 1, column + 3),
            self.get_index(row - 1, column + 4), // First line
            self.get_index(row - 2, column + 1),
            self.get_index(row - 3, column + 1),
            self.get_index(row - 4, column + 1), // Second line
            self.get_index(row - 2, column + 6),
            self.get_index(row - 3, column + 6),
            self.get_index(row - 4, column + 6), // Third line (most left one)
            self.get_index(row - 6, column + 2),
            self.get_index(row - 6, column + 3),
            self.get_index(row - 6, column + 4), // Fourth line (most top one)
            // Third pattern at bottom left
            self.get_index(row + 1, column - 2),
            self.get_index(row + 1, column - 3),
            self.get_index(row + 1, column - 4), // First line
            self.get_index(row + 2, column - 1),
            self.get_index(row + 3, column - 1),
            self.get_index(row + 4, column - 1), // Second line
            self.get_index(row + 2, column - 6),
            self.get_index(row + 3, column - 6),
            self.get_index(row + 4, column - 6), // Third line (most left one)
            self.get_index(row + 6, column - 2),
            self.get_index(row + 6, column - 3),
            self.get_index(row + 6, column - 4), // Fourth line (most top one)
            // Fourth pattern at bottom right
            self.get_index(row + 1, column + 2),
            self.get_index(row + 1, column + 3),
            self.get_index(row + 1, column + 4), // First line
            self.get_index(row + 2, column + 1),
            self.get_index(row + 3, column + 1),
            self.get_index(row + 4, column + 1), // Second line
            self.get_index(row + 2, column + 6),
            self.get_index(row + 3, column + 6),
            self.get_index(row + 4, column + 6), // Third line (most left one)
            self.get_index(row + 6, column + 2),
            self.get_index(row + 6, column + 3),
            self.get_index(row + 6, column + 4), // Fourth line (most top one)
        ];

        for idx in pulsar_indexes {
            self.cells[idx] = true;
        }
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = match cell {
                    true => "◼",
                    false => "◻",
                };

                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
