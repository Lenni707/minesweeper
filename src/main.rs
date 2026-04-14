use macroquad::prelude::*;
use ::rand::{RngExt, SeedableRng, rngs::StdRng};

// --- TODO ---
// menu and win + loose animation
// option to reset
// timer

const GRID_WIDTH: usize = 20;
const GRID_HEIGHT: usize = 20;
const CELL_SIZE: usize = 40;

const NUM_BOMBS: i32 = ((GRID_HEIGHT as f32 * GRID_WIDTH as f32) * 0.25) as i32;

const SEED: u64 = 676767;

struct Assets {
    bomb: Texture2D,
    flag: Texture2D,
}

impl Assets {
    async fn load() -> Self {
        let bomb = load_texture("assets/sprites_png/bomb.png")
            .await
            .unwrap();
        bomb.set_filter(FilterMode::Nearest);
        
        let flag = load_texture("assets/sprites_png/flag.png")
            .await
            .unwrap();
        flag.set_filter(FilterMode::Nearest);

        Self {
            bomb,
            flag
        }
    }
}

struct World {
    grid: Vec<Vec<Cell>>,
    cell_size: usize,
    generated: bool,
    rng: StdRng,
    num_flags: u32,
}

#[derive(Clone, Copy, PartialEq)]
enum CellType {
    Mine,
    Number(u32),
    Empty
}

#[derive(Clone, Copy, PartialEq)]
struct Cell {
    kind: CellType,
    revealed: bool,
    flagged: bool
}

impl Cell {
    fn new(cell_type: CellType) -> Self {
        Cell { kind: cell_type, revealed: false, flagged: false }
    }
}

impl World {
    fn new() -> Self {
        let empty_grid = vec![vec![Cell::new(CellType::Empty); GRID_WIDTH]; GRID_HEIGHT];

        World { grid: empty_grid, cell_size: CELL_SIZE, generated: false, rng: StdRng::seed_from_u64(SEED), num_flags: 0 }
    }
    fn generate(&mut self, safe_x: usize, safe_y: usize) {
        let num_neighbouring_tiles_that_are_also_safe = 2; // normally 1
        // get the exluded cells from the first click
        let mut excluded = std::collections::HashSet::new();
        for dy in -num_neighbouring_tiles_that_are_also_safe..=num_neighbouring_tiles_that_are_also_safe {
            for dx in -num_neighbouring_tiles_that_are_also_safe..=num_neighbouring_tiles_that_are_also_safe {
                let nx = safe_x as isize + dx;
                let ny = safe_y as isize + dy;
                if is_in_bounds(nx, ny) {
                    excluded.insert((nx as usize, ny as usize));
                }
            }
        }

        // gen mines
        let mut count = 0;
        while count < NUM_BOMBS {
            let x = self.rng.random_range(0..GRID_WIDTH);
            let y = self.rng.random_range(0..GRID_HEIGHT);
            if self.grid[y][x].kind == CellType::Empty && !excluded.contains(&(x, y)) {
                self.grid[y][x].kind = CellType::Mine;
                count += 1;
            }
        }

        // gen numbers based on mines
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let n = get_num_neighbor_mines(&self.grid, x, y);
                if self.grid[y][x].kind == CellType::Empty && n > 0 {
                    self.grid[y][x].kind = CellType::Number(n);
                }
            }
        }

        self.generated = true;
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
                if grid[ny as usize][nx as usize].kind == CellType::Mine {
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

fn world_to_grid(world_x: f32, world_y: f32) -> Option<(usize, usize)> {
        let gx = (world_x / CELL_SIZE as f32) as isize;
        let gy = (world_y / CELL_SIZE as f32) as isize;
        is_in_bounds(gx, gy).then_some((gx as usize, gy as usize))
}

fn flood_fill(grid: &mut Vec<Vec<Cell>>, start_x: usize, start_y: usize) {
    let mut queue = std::collections::VecDeque::new();
    queue.push_back((start_x, start_y));

    while let Some((x, y)) = queue.pop_front() {
        let cell = &mut grid[y][x];

        // skip if already revealed or flagged
        if cell.revealed || cell.flagged {
            continue;
        }

        cell.revealed = true;

        // only spread further from empty cells
        // number cells reveal themselves but don't propagate
        if cell.kind != CellType::Empty {
            continue;
        }

        // push all valid neighbors
        for dy in -1..=1isize {
            for dx in -1..=1isize {
                if dx == 0 && dy == 0 { continue; }
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if is_in_bounds(nx, ny) {
                    queue.push_back((nx as usize, ny as usize));
                }
            }
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Minesweeper".to_owned(),
        window_width: (((GRID_WIDTH * CELL_SIZE) as f32) * 1.5) as i32, // hahah dieses * 1.5 ist so dumm aber sosnt ist es zu klein idk wieso
        window_height: (((GRID_HEIGHT * CELL_SIZE) as f32) * 1.5) as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)] 
async fn main() {
    let mut world = World::new();

    let assets = Assets::load().await;

    loop {
        handle_mouse(&mut world);
        draw(&world, &assets);
        next_frame().await
    }
}

fn handle_mouse(world: &mut World) {
    if is_mouse_button_pressed(MouseButton::Left) {
        let (mx, my) = mouse_position();
        if let Some((gx, gy)) = world_to_grid(mx, my) {
            if !world.generated {
                world.generate( gx, gy);
            } else {
                match world.grid[gy][gx].kind {
                    CellType::Mine => { panic!("you lost hahahah") },
                    CellType::Empty => { flood_fill(&mut world.grid, gx, gy) },
                    CellType::Number(_) => { world.grid[gy][gx].revealed = true },
                }
            }
        }
    }
    if is_mouse_button_pressed(MouseButton::Right) {
        let (mx, my) = mouse_position();
        if let Some((gx, gy)) = world_to_grid(mx, my) {
            // world.grid[gy][gx].flagged = !world.grid[gy][gx].flagged; so viel cooler
            if world.grid[gy][gx].revealed {
                return
            }

            if world.grid[gy][gx].flagged {
                world.grid[gy][gx].flagged = false;
                world.num_flags -= 1;                
            } else {
                world.grid[gy][gx].flagged = true;
                world.num_flags += 1;  
            }
        }
    }
}

fn draw(world: &World, assets: &Assets) {
    clear_background(GRAY);
    draw_cells(world, assets);
    draw_grid_lines(world);

    draw_text(&(NUM_BOMBS - world.num_flags as i32).to_string(), 10., 40., 40., WHITE); // show num bombs
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

fn draw_cells(grid: &World, assets: &Assets) {
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let cell = grid.grid[y][x];
            let x_pos = (x * grid.cell_size) as f32;
            let y_pos = (y * grid.cell_size) as f32;
            let size = grid.cell_size as f32;

            if !cell.revealed {
                draw_rectangle(x_pos, y_pos, size, size, DARKGRAY);
                if cell.flagged {
                    // draw flag indicator on top
                    draw_texture_ex(
                        &assets.flag,
                        x_pos,
                        y_pos,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(grid.cell_size as f32, grid.cell_size as f32)), // integer scale
                            ..Default::default()
                        },
                    );
                }
                continue;
            }

            match cell.kind {
                CellType::Mine => { 
                    draw_texture_ex(
                        &assets.bomb,
                        x_pos,
                        y_pos,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(grid.cell_size as f32, grid.cell_size as f32)), // integer scale
                            ..Default::default()
                        },
                    );
                },
                CellType::Number(n) => {
                    let color = match n {
                        1 => DARKBLUE,
                        2 => DARKGREEN,
                        3 => RED,
                        4 => DARKPURPLE,
                        5 => YELLOW,
                        6 => LIME,
                        7 => PINK,
                        8 => ORANGE,
                        _ => WHITE // shouldnt be possible
                    };
                    draw_rectangle(x_pos, y_pos, size, size, GRAY);
                    draw_text(&n.to_string(), x_pos + size/4.0, y_pos + size*0.75, size*1.2, color);
                }
                CellType::Empty => draw_rectangle(x_pos, y_pos, size, size, GRAY),
            }
        }
    }
}