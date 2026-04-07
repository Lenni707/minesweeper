use macroquad::prelude::*;
use ::rand::{Rng, SeedableRng};

const GRID_WIDTH: usize = 30;
const GRID_HEIGHT: usize = 30;



struct World {
    grid: Vec<Vec<Cell>>
}

#[derive(Clone, Copy)]
enum Cell {
    Mine,
    Field(u32),
    Flag,
    Hidden
}

impl World {
    fn new() {
        
    }
    fn set_cell_at(&mut self, cell: Cell, x: usize, y: usize) {
        if y < GRID_HEIGHT && x < GRID_WIDTH {
            self.grid[y][x] = cell;
        }
    }
    fn get_cell_at(&self, x: usize, y: usize) -> Option<&Cell> {
        self.grid.get(y)?.get(x)
    }
}

#[macroquad::main("Minesweeper")]
async fn main() {
    loop {
        draw();


        next_frame().await
    }
}

fn draw() {
    clear_background(BLACK);
}