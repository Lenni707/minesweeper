use macroquad::prelude::*;

const GRID_WIDTH: usize = 15;
const GRID_HEIGHT: usize = 15;
const CELL_SIZE: usize = 40;
const GRID_OFFSET_X: f32 = 0.0;
const GRID_OFFSET_Y: f32 = 80.0;

const NUM_BOMBS: i32 = ((GRID_HEIGHT as f32 * GRID_WIDTH as f32) * 0.25).round() as i32;

enum Scene {
    StartMenu,
    Game,
    WinScreen,
    LooseScreen
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

 struct World {
    grid: Vec<Vec<Cell>>,
    cell_size: usize,
    generated: bool,
    revealed: bool,
    seed: u64,
    num_flags: u32,
    amt_numbers: u32,
    amt_revelead_numbers: u32,
}

struct Assets {
    bomb: Texture2D,
    flag: Texture2D,
    flag_wrong: Texture2D,
    flag_correct: Texture2D,
}

impl Cell {                
    fn new(cell_type: CellType) -> Self {
        Cell { kind: cell_type, revealed: false, flagged: false }
    }
}

impl World {
    fn new(seed: u64) -> Self {
        let empty_grid = vec![vec![Cell::new(CellType::Empty); GRID_WIDTH]; GRID_HEIGHT];
        rand::srand(seed);
        World { grid: empty_grid, cell_size: CELL_SIZE, generated: false, revealed: false, seed, num_flags: 0, amt_numbers: 0, amt_revelead_numbers: 0  }
    }

    fn generate(&mut self, safe_x: usize, safe_y: usize) {
        let num_neighbouring_tiles_that_are_also_safe = 2;
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

        let mut count = 0;
        while count < NUM_BOMBS {
            let x = rand::gen_range(0, GRID_WIDTH);
            let y = rand::gen_range(0, GRID_HEIGHT);
            if self.grid[y][x].kind == CellType::Empty && !excluded.contains(&(x, y)) {
                self.grid[y][x].kind = CellType::Mine;
                count += 1;
            }
        }

        let mut count_nums = 0;
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let n = get_num_neighbor_mines(&self.grid, x, y);
                if self.grid[y][x].kind == CellType::Empty && n > 0 {
                    self.grid[y][x].kind = CellType::Number(n);
                    count_nums += 1
                }
            }
        }
        self.amt_numbers = count_nums;

        self.generated = true;
    }
}

impl Assets {
    async fn load() -> Result<Self, String> {
        let bomb = load_texture("assets/sprites_png/bomb.png")
            .await
            .map_err(|e| format!("Failed to load bomb.png: {:?}", e))?;
        bomb.set_filter(FilterMode::Nearest);
        
        let flag = load_texture("assets/sprites_png/flag.png")
            .await
            .map_err(|e| format!("Failed to load flag.png: {:?}", e))?;
        flag.set_filter(FilterMode::Nearest);

        let flag_wrong = load_texture("assets/sprites_png/flag_wrong.png")
            .await
            .map_err(|e| format!("Failed to load flag_wrong.png: {:?}", e))?;
        flag_wrong.set_filter(FilterMode::Nearest);

        let flag_correct = load_texture("assets/sprites_png/flag_correc_green.png")
            .await
            .map_err(|e| format!("Failed to load flag_correct.png: {:?}", e))?;
        flag_correct.set_filter(FilterMode::Nearest);

        Ok(Self { bomb, flag, flag_wrong, flag_correct })
    }
}

fn draw_centered_text(text: &str, y: f32, font_size: f32, color: Color) {
    let dimensions = measure_text(text, None, font_size as u16, 1.0);
    let x = (screen_width() - dimensions.width) / 2.0;
    draw_text(text, x, y, font_size, color);
}

fn check_win(world: &World) -> bool {
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let cell = &world.grid[y][x];
            if cell.kind != CellType::Mine && !cell.revealed {
                return false;
            }
        }
    }
    true
}

fn start_menu(world: &mut World) -> Scene {
    clear_background(Color::from_rgba(20, 20, 25, 255));

    draw_centered_text("MINESWEEPER", 180.0, 50.0, ORANGE);
    
    draw_centered_text("Controls", 300.0, 30.0, LIGHTGRAY);
    draw_centered_text("Left Click: Reveal tile", 350.0, 24.0, WHITE);
    draw_centered_text("Right Click: Toggle flag", 390.0, 24.0, WHITE);
    draw_centered_text("R Key: Reset game", 430.0, 24.0, WHITE);
    
    let pulse = (get_time() * 3.0).sin() as f32;
    let start_color = if pulse > 0.0 { WHITE } else { GRAY };
    draw_centered_text("Press SPACE to start", 550.0, 28.0, start_color);

    if is_key_pressed(KeyCode::Space) {
        *world = World::new(rand::gen_range(0, 99999));
        return Scene::Game;
    }

    Scene::StartMenu
}

fn play_game(world: &mut World, assets: &Assets) -> Scene {
    let next_scene = handle_mouse(world);
    draw(world, assets);

    if let Some(scene) = next_scene {
        return scene;
    }

    if is_key_pressed(KeyCode::R) {
        return Scene::StartMenu;
    }

    if check_win(world) {
        return Scene::WinScreen;
    }

    Scene::Game
}

fn win_menu(world: &mut World, assets: &Assets) -> Scene {
    draw(world, assets);

    draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.7));

    draw_centered_text("VICTORY!", screen_height() / 2.0 - 40.0, 60.0, GREEN);
    draw_centered_text("You cleared all mines!", screen_height() / 2.0 + 20.0, 24.0, WHITE);
    draw_centered_text("Press R to start a new game", screen_height() / 2.0 + 70.0, 24.0, LIGHTGRAY);

    if is_key_pressed(KeyCode::R) {
        return Scene::StartMenu;
    }

    Scene::WinScreen
}

fn loose_menu(world: &mut World, assets: &Assets) -> Scene {
    reveal(world);
    draw(world, assets);

    draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.2));

    draw_centered_text("GAME OVER", screen_height() / 2.0 - 40.0, 60.0, RED);
    draw_centered_text("You stepped on a mine!", screen_height() / 2.0 + 20.0, 24.0, WHITE);
    draw_centered_text("Press R to start a new game", screen_height() / 2.0 + 70.0, 24.0, LIGHTGRAY);

    if is_key_pressed(KeyCode::R) {
        return Scene::StartMenu;
    }

    Scene::LooseScreen
}

fn handle_mouse(world: &mut World) -> Option<Scene> { // for handling on a loose
    if world.revealed {
        return None;
    }

    if is_mouse_button_pressed(MouseButton::Left) {
        let (mx, my) = mouse_position();
        if let Some((gx, gy)) = world_to_grid(mx, my) {
            if !world.generated {
                world.generate(gx, gy);
                flood_fill(&mut world.grid, gx, gy);
            } else if !world.grid[gy][gx].flagged {
                match world.grid[gy][gx].kind {
                    CellType::Mine => { return Some(Scene::LooseScreen) },
                    CellType::Empty => { flood_fill(&mut world.grid, gx, gy);  },
                    CellType::Number(_) => { world.grid[gy][gx].revealed = true; world.amt_revelead_numbers += 1 },
                }
            }
        }
    }

    if is_mouse_button_pressed(MouseButton::Right) {
        let (mx, my) = mouse_position();
        if let Some((gx, gy)) = world_to_grid(mx, my) {
            if world.grid[gy][gx].revealed {
                return None
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

    None
}

fn draw(world: &World, assets: &Assets) {
    clear_background(GRAY);
    draw_cells(world, assets);
    draw_grid_lines(world);

    draw_text(
        &format!("Bombs remaining: {}", NUM_BOMBS - world.num_flags as i32),
        10.,
        33.,
        (GRID_HEIGHT * 2) as f32,
        WHITE,
    );

    draw_text(&format!("Seed: {}", world.seed), 10., 70., (GRID_HEIGHT * 2) as f32, WHITE);
}

fn draw_cells(grid: &World, assets: &Assets) {
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let cell = grid.grid[y][x];
            let x_pos = (x * grid.cell_size) as f32 + GRID_OFFSET_X;
            let y_pos = (y * grid.cell_size) as f32 + GRID_OFFSET_Y;
            let size = grid.cell_size as f32;

            if !cell.revealed {
                draw_rectangle(x_pos, y_pos, size, size, DARKGRAY);
            } else {
                match cell.kind {
                    CellType::Mine => { 
                        if !cell.flagged {
                            draw_texture_ex(
                                &assets.bomb,
                                x_pos,
                                y_pos,
                                WHITE,
                                DrawTextureParams {
                                    dest_size: Some(vec2(grid.cell_size as f32, grid.cell_size as f32)),
                                    ..Default::default()
                                },
                            );
                        }
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
                            _ => WHITE
                        };
                        draw_rectangle(x_pos, y_pos, size, size, GRAY);
                        draw_text(&n.to_string(), x_pos + size/4.0, y_pos + size*0.75, size*1.2, color);
                    }
                    CellType::Empty => draw_rectangle(x_pos, y_pos, size, size, GRAY),
                }
            }

            if cell.flagged {
                let texture = if grid.revealed {
                    match cell.kind {
                        CellType::Mine => &assets.flag_correct,
                        _ => &assets.flag_wrong,
                    }
                } else {
                    &assets.flag
                };

                draw_texture_ex(
                    texture,
                    x_pos,
                    y_pos,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(grid.cell_size as f32, grid.cell_size as f32)),
                        ..Default::default()
                    },
                );
            }
        }
    }
}

fn draw_grid_lines(grid: &World) {
    let width = GRID_WIDTH * grid.cell_size;
    let height = GRID_HEIGHT * grid.cell_size;

    for x in 0..=GRID_WIDTH {
        let x_pos = x * grid.cell_size;
        draw_line(x_pos as f32 + GRID_OFFSET_X, GRID_OFFSET_Y, x_pos as f32 + GRID_OFFSET_X, height as f32 + GRID_OFFSET_Y, 1.0, WHITE);
    }

    for y in 0..=GRID_HEIGHT {
        let y_pos = y * grid.cell_size;
        draw_line(GRID_OFFSET_X, y_pos as f32 + GRID_OFFSET_Y, width as f32 + GRID_OFFSET_X, y_pos as f32 + GRID_OFFSET_Y, 1.0, WHITE);
    }
}

fn reveal(world: &mut World) {
    world.revealed = true;

    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let cell = &mut world.grid[y][x];
            cell.revealed = true;
        }
    }
}

fn flood_fill(grid: &mut Vec<Vec<Cell>>, start_x: usize, start_y: usize) {
    let mut queue = std::collections::VecDeque::new();
    queue.push_back((start_x, start_y));

    while let Some((x, y)) = queue.pop_front() {
        let cell = &mut grid[y][x];

        if cell.revealed || cell.flagged {
            continue;
        }

        cell.revealed = true;

        if cell.kind != CellType::Empty {
            continue;
        }

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

fn get_num_neighbor_mines(grid: &Vec<Vec<Cell>>, x: usize, y: usize) -> u32 {
    let mut mine_neighbours: u32 = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
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

fn world_to_grid(world_x: f32, world_y: f32) -> Option<(usize, usize)> {
    let gx = ((world_x - GRID_OFFSET_X) / CELL_SIZE as f32) as isize;
    let gy = ((world_y - GRID_OFFSET_Y) / CELL_SIZE as f32) as isize;
    is_in_bounds(gx, gy).then_some((gx as usize, gy as usize))
}

fn is_in_bounds(x: isize, y: isize) -> bool {
    x >= 0 && x < GRID_WIDTH as isize && y >= 0 && y < GRID_HEIGHT as isize
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Minesweeper".to_owned(),
        window_width: (((GRID_WIDTH * CELL_SIZE) as f32 + GRID_OFFSET_X * 2.0) * 1.) as i32,
        window_height: (((GRID_HEIGHT * CELL_SIZE) as f32 + GRID_OFFSET_Y + GRID_OFFSET_X) * 1.) as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)] 
async fn main() {
    let seed: u64 = rand::gen_range(0, 99999);
    let mut world = World::new(seed);
    let assets = match Assets::load().await {
        Ok(a) => a,
        Err(err) => {
            eprintln!("{}", err); 
            panic!("{}", err);
        }
    };
    let mut scene = Scene::StartMenu;

    loop {
        scene = match scene {
            Scene::StartMenu => start_menu(&mut world),
            Scene::Game => play_game(&mut world, &assets),
            Scene::WinScreen => win_menu(&mut world, &assets),
            Scene::LooseScreen => loose_menu(&mut world, &assets)
        };
        next_frame().await
    }
}