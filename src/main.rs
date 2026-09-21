use ::rand::random_range;
use image::{
    Delay, Frame,
    codecs::gif::{GifEncoder, Repeat},
};
use macroquad::{input::KeyCode::Space, prelude::*};
use std::fs::File;
const SCREEN_SIZE: (u32, u32) = (700, 700); //(x,y)
const BG_COLOR: Color = BLACK;
const FULL_SCREEN: bool = false;
const WAIT_FOR_SIGNAL: bool = true;

const EXPORTING: bool = false;
const EXPORTING_RATE: u16 = 5;
const SCREEN_LIMIT: u16 = 200;
const DELAY: u8 = 20;
const EXPORT_DIR: &str = "/home/cookie/Documents/Dev/Rust/Bordel Evolutif/gof_v2/Frames/frame";

const SPAWN_TYPE: SpawnType = SpawnType::FullRandom;
const SPAWN_RATE: u8 = 6;
const CIRCLE_ROUND: u32 = 70;
const CIRCLE_WEIGHT: u32 = 20;
enum SpawnType {
    FullRandom,
    FullCircle,
}
//  Classic : b: 2,3 s: 3
const CELL_TYPE: CellType = CellType {
    b: &[2, 3, 4],
    s: &[],
    color: YELLOW,
};

#[derive(PartialEq, Copy, Clone)]
enum CellState {
    Alive,
    Dead,
}

struct CellType {
    b: &'static [u8],
    s: &'static [u8],
    color: Color,
}
impl CellType {
    fn check_surviving(&self, actual_state: CellState, score: u8) -> CellState {
        match actual_state {
            CellState::Alive => {
                for s in self.s {
                    if s == &score {
                        return CellState::Alive;
                    }
                }
                return CellState::Dead;
            }
            CellState::Dead => {
                for b in self.b {
                    if b == &score {
                        return CellState::Alive;
                    }
                }
                CellState::Dead
            }
        }
    }
}
fn init_pop() -> Vec<CellState> {
    let mut population: Vec<CellState> = vec![];
    match SPAWN_TYPE {
        SpawnType::FullRandom => {
            for _cra in 0..SCREEN_SIZE.0 * SCREEN_SIZE.1 {
                if random_range(0..SPAWN_RATE) == 0 {
                    population.push(CellState::Alive);
                } else {
                    population.push(CellState::Dead);
                }
            }
        }
        SpawnType::FullCircle => {
            let cx = (SCREEN_SIZE.0 / 2) as i32;
            let cy = (SCREEN_SIZE.1 / 2) as i32;
            let r = CIRCLE_ROUND as i32;
            for cra in 0..SCREEN_SIZE.0 * SCREEN_SIZE.1 {
                let dx = (cra % SCREEN_SIZE.0) as i32 - cx;
                let dy = (cra / SCREEN_SIZE.0) as i32 - cy;
                if dx * dx + dy * dy <= r * r
                    && dx * dx + dy * dy >= (CIRCLE_WEIGHT * CIRCLE_WEIGHT) as i32
                {
                    population.push(CellState::Alive);
                } else {
                    population.push(CellState::Dead);
                }
            }
        }
    }

    population
}

fn ptinx(pos_x: u32, pos_y: u32) -> u32 {
    pos_y * SCREEN_SIZE.0 + pos_x
}
// fn inxtp(inx: u32) -> (u32, u32) {
//     (inx % SCREEN_SIZE.0, inx / SCREEN_SIZE.0)
// }
fn scan_cell(cell_pos: (u32, u32), popu: &[CellState]) -> CellState {
    let mut score: u8 = 0;
    for y in -1..=1 {
        for x in -1..=1 as i32 {
            if cell_pos.0 as i32 + x < 0
                || cell_pos.0 as i32 + x >= SCREEN_SIZE.0 as i32
                || cell_pos.1 as i32 + y >= SCREEN_SIZE.1 as i32
                || cell_pos.1 as i32 + y < 0
                || (x == 0 && y == 0)
            {
                continue;
            }
            let target_inx = ptinx(
                (cell_pos.0 as i32 + x) as u32,
                (cell_pos.1 as i32 + y) as u32,
            );

            if popu[target_inx as usize] == CellState::Alive {
                score += 1;
            }
        }
    }

    let truc_state = popu[ptinx(cell_pos.0, cell_pos.1) as usize];

    return CELL_TYPE.check_surviving(truc_state, score);
}

#[macroquad::main("Game of life (boosted)")]
async fn main() {
    let mut instance: u64 = 0;
    let mut population: Vec<CellState> = init_pop();
    request_new_screen_size(SCREEN_SIZE.0 as f32, SCREEN_SIZE.1 as f32);
    set_fullscreen(FULL_SCREEN);
    let file = File::create("output.gif").unwrap();
    let mut encoder = GifEncoder::new_with_speed(file, 10);
    encoder.set_repeat(Repeat::Infinite).unwrap();

    if WAIT_FOR_SIGNAL {
        while !is_key_down(Space) {
            next_frame().await
        }
    }
    let mut captured = 0;

    loop {
        let mut image_frame =
            Image::gen_image_color(SCREEN_SIZE.0 as u16, SCREEN_SIZE.1 as u16, BG_COLOR);
        let buffer_pop: Vec<CellState> = population.clone();
        for y in 0..SCREEN_SIZE.1 {
            for x in 0..SCREEN_SIZE.0 {
                let cell_state = scan_cell((x, y), &buffer_pop);
                population[ptinx(x, y) as usize] = cell_state;
                if cell_state == CellState::Alive {
                    image_frame.set_pixel(x, y, CELL_TYPE.color);
                };
            }
        }
        if instance % EXPORTING_RATE as u64 == 0 && captured < SCREEN_LIMIT && EXPORTING {
            captured += 1;
            image_frame.export_png(format!("{EXPORT_DIR}_{instance}.png").as_str());
            //println!("screenshit {instance}");
            let img = image::open(format!("{EXPORT_DIR}_{instance}.png"))
                .unwrap()
                .to_rgba8();

            encoder
                .encode_frame(Frame::from_parts(
                    img,
                    0,
                    0,
                    Delay::from_numer_denom_ms(DELAY as u32, 1),
                ))
                .unwrap();
        } else if SCREEN_LIMIT <= captured {
            break;
        }
        instance += 1;
        draw_texture(
            &macroquad::texture::Texture2D::from_image(&image_frame),
            0.0,
            0.0,
            WHITE,
        );
        next_frame().await;
    }
}
