use ::rand::random_range;
use macroquad::prelude::*;

const WIDHT: usize = 800;
const HEIGHT: usize = 800;
const SPAWN_RATE: u8 = 4;

#[derive(Clone, PartialEq, Copy)]
enum CellState {
    Alive,
    Dead,
}

#[macroquad::main("Game of Life")]
async fn main() {
    request_new_screen_size(WIDHT as f32, HEIGHT as f32);
    // Init cells
    let mut cells = vec![CellState::Dead; WIDHT * HEIGHT];
    for who in 0..WIDHT * HEIGHT {
        if random_range(0..SPAWN_RATE) == 0 {
            cells[who] = CellState::Alive;
        }
    }
    while !is_key_down(KeyCode::Space) {
        next_frame().await
    }
    loop {
        clear_background(BLACK);
        let mut buffer = cells.clone();
        for y in 0..HEIGHT as isize {
            for x in 0..WIDHT as isize {
                let mut neighbours_nb: u8 = 0;
                for i in -1..2 {
                    for j in -1..2 as isize {
                        if x + j < 0
                            || x + j >= WIDHT as isize
                            || y + i < 0
                            || y + i >= HEIGHT as isize
                            || (i == 0 && j == 0)
                        // || x * y >= (HEIGHT * WIDHT) as isize
                        {
                            continue;
                        }
                        //println!("Case : {}, x {x} y {y}", y * HEIGHT as isize + x + i + j);
                        if cells[(y * HEIGHT as isize + x + i * WIDHT as isize + j) as usize]
                            == CellState::Alive
                        {
                            neighbours_nb += 1
                        }
                    }
                }
                let actual_cell = (y * HEIGHT as isize + x) as usize;
                buffer[actual_cell] = match (cells[actual_cell], neighbours_nb) {
                    (CellState::Alive, nb) if nb == 2 || nb == 3 => {
                        draw_rectangle(x as f32, y as f32, 1 as f32, 1 as f32, WHITE);
                        CellState::Alive
                    }
                    (CellState::Dead, nb) if nb == 3 => {
                        draw_rectangle(x as f32, y as f32, 1 as f32, 1 as f32, WHITE);
                        CellState::Alive
                    }
                    (_a, _x) => CellState::Dead,
                };
            }
        }
        cells = buffer;
        next_frame().await
    }
}
