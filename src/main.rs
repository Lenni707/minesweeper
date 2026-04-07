use macroquad::prelude::*;
use ::rand::{Rng, SeedableRng};

const GRID_WIDTH: usize = 30;
const GRID_HEIGHT: usize = 30;
const CELL_SIZE: usize = 10;

struct World {
    grid: Vec<Vec<Cell>>,
    cell_size: usize,
}

#[derive(Clone, Copy)]
enum Cell {
    Mine,
    Field(u32),
    Flag,
    Hidden
}

impl World {
    fn new() -> Self {
        World { grid: vec![vec![Cell::Hidden; GRID_WIDTH]; GRID_HEIGHT], cell_size: CELL_SIZE }
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

fn window_conf() -> Conf {
    Conf {
        window_title: "Minesweeper".to_owned(),
        window_width: (GRID_WIDTH * CELL_SIZE) as i32,
        window_height: (GRID_HEIGHT * CELL_SIZE) as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let grid = World::new();
    loop {
        draw(&grid);


        next_frame().await
    }
}

fn draw(grid: &World) {

    clear_background(BLACK);

    draw_grid_lines(grid);
}

fn draw_grid_lines(grid: &World) {
    let width = GRID_WIDTH * grid.cell_size;
    let height = GRID_HEIGHT * grid.cell_size;

    for x in 0..=GRID_WIDTH {
        let x_pos = x * grid.cell_size;
        draw_line(x_pos as f32, 0., x_pos as f32, height as f32, 1.0, WHITE);
    }

    for y in 0..=GRID_HEIGHT {
        let y_pos = y * grid.cell_size;
        draw_line(0., y_pos as f32, width as f32, y_pos as f32, 1.0, WHITE);
    }
}