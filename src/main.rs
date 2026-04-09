use core::num;

use macroquad::prelude::*;
use ::rand::{RngExt, SeedableRng, rngs::StdRng};

// --- TODO ---
// - Code dick optimieren (vorallem loops, weil ich durch vieles einfach viel zu oft looper)
// actually playable machen (clickable und das man felder aufdeckt)
// maybe cell sache rewritten
// jede mine muss mindesten ein number feld als nachbarn haben
// generell mit dem design diesmal mühe geben

const GRID_WIDTH: usize = 30;
const GRID_HEIGHT: usize = 30;
const CELL_SIZE: usize = 20;

const NUM_BOMBS: i32 = ((GRID_HEIGHT as f32 * GRID_WIDTH as f32) * 0.3) as i32;

const SEED: u64 = 12345;

struct World {
    grid: Vec<Vec<Cell>>,
    cell_size: usize,
}

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Mine,
    Number(u32),
    Flag,
    Empty
}

impl World {
    fn new(mut rng: StdRng) -> Self {
        let mut grid = vec![vec![Cell::Empty; GRID_WIDTH]; GRID_HEIGHT];

        // gen bombs das ist glaub ich gut so der rest ist cooked, bin todes müde
        let mut count = 0;
        while count < NUM_BOMBS {
            let x = rng.random_range(0..GRID_WIDTH);
            let y = rng.random_range(0..GRID_HEIGHT);

            if grid[y][x] == Cell::Empty {
                grid[y][x] = Cell::Mine;
                count += 1;
            }
        }

        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let num_neighbour_mines = get_num_neighbor_mines(&grid, x, y);
                if grid[y][x] != Cell::Empty || num_neighbour_mines < 1 {
                    continue;
                }
                grid[y][x] = Cell::Number(num_neighbour_mines)
            }
        }

        World { grid, cell_size: CELL_SIZE }
    }
    fn set_cell_at(&mut self, cell: Cell, x: usize, y: usize) {
        if y < GRID_HEIGHT && x < GRID_WIDTH {
            self.grid[y][x] = cell;
        }
    }

}

fn get_num_neighbor_mines(grid: &Vec<Vec<Cell>>, x: usize, y: usize) -> u32 {
    let mut mine_neighbours: u32 = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            // if its the cell itself
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if is_in_bounds(nx, ny) {
                if grid[ny as usize][nx as usize] == Cell::Mine {
                    mine_neighbours += 1
                }
            }
        }
    }
    mine_neighbours
}

fn is_in_bounds(x: isize, y: isize) -> bool {
    x >= 0 && x < GRID_WIDTH as isize && y >= 0 && y < GRID_HEIGHT as isize
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
    let rng = StdRng::seed_from_u64(SEED);
    let grid = World::new(rng);
    loop {
        draw(&grid);


        next_frame().await
    }
}

fn draw(grid: &World) {
    clear_background(BLACK);
    draw_cells(grid);
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

fn draw_cells(grid: &World) {
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let cell = grid.grid[y][x];
            let x_pos = (x * grid.cell_size) as f32;
            let y_pos = (y * grid.cell_size) as f32;
            let size = grid.cell_size as f32;

            match cell {
                Cell::Mine => {
                    draw_rectangle(x_pos, y_pos, size, size, RED);
                }
                Cell::Number(n) => {
                    draw_rectangle(x_pos, y_pos, size, size, GRAY);
                    draw_text(
                        &n.to_string(),
                        x_pos + size / 4.0,
                        y_pos + size * 0.75,
                        size * 0.8,
                        WHITE,
                    );
                }
                Cell::Empty => {
                    draw_rectangle(x_pos, y_pos, size, size, DARKGRAY);
                }
                Cell::Flag => {
                    draw_rectangle(x_pos, y_pos, size, size, BLUE);
                }
            }
        }
    }
}