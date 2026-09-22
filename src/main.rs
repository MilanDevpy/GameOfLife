mod config;
use config::*;

mod cyclic;
mod life;

use image::{
    Delay, Frame,
    codecs::gif::{GifEncoder, Repeat},
};

use ::rand::random;
use macroquad::{input::KeyCode::Space, prelude::*};
use std::fs::File;

fn ptinx(pos_x: u32, pos_y: u32) -> u32 {
    pos_y * SCREEN_SIZE.0 + pos_x
}
// fn inxtp(inx: u32) -> (u32, u32) {
//     (inx % SCREEN_SIZE.0, inx / SCREEN_SIZE.0)
// }

#[macroquad::main("Game of life (boosted)")]
async fn main() {
    let mut instance: u64 = 0;

    request_new_screen_size(SCREEN_SIZE.0 as f32, SCREEN_SIZE.1 as f32);
    set_fullscreen(FULL_SCREEN);
    let file: File;

    file = File::create("output.gif").unwrap();
    let mut encoder = GifEncoder::new_with_speed(file, 10);
    encoder.set_repeat(Repeat::Infinite).unwrap();

    if WAIT_FOR_SIGNAL {
        while !is_key_down(Space) {
            next_frame().await
        }
    }
    let mut captured = 0;
    match SIMULATION_TYPE {
        SimulationType::Life => {
            let mut population: Vec<CellState> = life::init_pop();
            loop {
                let mut image_frame =
                    Image::gen_image_color(SCREEN_SIZE.0 as u16, SCREEN_SIZE.1 as u16, BG_COLOR);
                let buffer_pop: Vec<CellState> = population.clone();
                for y in 0..SCREEN_SIZE.1 {
                    for x in 0..SCREEN_SIZE.0 {
                        let cell_state = life::scan_cell((x, y), &buffer_pop);
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
        SimulationType::Cyclic => {
            let mut population: Vec<u8> = cyclic::init_pop();
            loop {
                let mut image_frame =
                    Image::gen_image_color(SCREEN_SIZE.0 as u16, SCREEN_SIZE.1 as u16, BG_COLOR);
                let buffer_pop = population.clone();
                for y in 0..SCREEN_SIZE.1 {
                    for x in 0..SCREEN_SIZE.0 {
                        let cell_state = cyclic::scan_cell((x, y), &buffer_pop);
                        population[ptinx(x, y) as usize] = cell_state;

                        image_frame.set_pixel(x, y, cyclic::state_to_color(cell_state));
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
    }
}
