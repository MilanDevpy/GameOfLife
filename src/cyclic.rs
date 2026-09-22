use macroquad::color::Color;
use rand::random_range;

use crate::config::*;
use crate::ptinx;

pub fn init_pop() -> Vec<u8> {
    let mut population: Vec<u8> = vec![];
    match SPAWN_TYPE {
        SpawnType::FullRandom => {
            for _cra in 0..SCREEN_SIZE.0 * SCREEN_SIZE.1 {
                population.push(random_range(0..CYCLIC_PATTERN.states));
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
                    population.push(random_range(0..CYCLIC_PATTERN.states));
                } else {
                    population.push(0);
                }
            }
        }
    }

    population
}

pub fn scan_cell(cell_pos: (u32, u32), popu: &[u8]) -> u8 {
    let mut score: u8 = 0;
    let next = (popu[ptinx(cell_pos.0, cell_pos.1) as usize] + 1) % CYCLIC_PATTERN.states;
    for y in CYCLIC_PATTERN.search_distance as i32 * -1..=CYCLIC_PATTERN.search_distance as i32 {
        for x in CYCLIC_PATTERN.search_distance as i32 * -1
            ..=CYCLIC_PATTERN.search_distance as i32 as i32
        {
            if x == 0 && y == 0 {
                continue;
            }

            let target_inx = ptinx(
                (cell_pos.0 as i32 + x).rem_euclid(SCREEN_SIZE.0 as i32) as u32,
                (cell_pos.1 as i32 + y).rem_euclid(SCREEN_SIZE.1 as i32) as u32,
            );

            if popu[target_inx as usize] == next {
                score += 1;
            }
        }
    }

    let cell_who = popu[ptinx(cell_pos.0, cell_pos.1) as usize];

    if score >= CYCLIC_PATTERN.neighbours {
        next
    } else {
        return cell_who;
    }
}

pub fn state_to_color(st: u8) -> Color {
    match CYCLIC_PATTERN.color_scheme {
        CyclicColors::BlueToPurple => match st {
            0 => Color {
                r: 6. / 255.,
                g: 0. / 255.,
                b: 50. / 255.,
                a: 1.,
            },
            1 => Color {
                r: 30. / 255.,
                g: 9. / 255.,
                b: 115. / 255.,
                a: 1.,
            },
            2 => Color {
                r: 0.,
                g: 0.,
                b: 0.,
                a: 1.,
            },
            3 => Color {
                r: 10. / 255.,
                g: 26. / 255.,
                b: 94. / 255.,
                a: 1.,
            },
            4 => Color {
                r: 148. / 255.,
                g: 20. / 255.,
                b: 239. / 255.,
                a: 1.,
            },
            5 => Color {
                r: 172. / 255.,
                g: 20. / 255.,
                b: 239. / 255.,
                a: 1.,
            },
            6 => Color {
                r: 219. / 255.,
                g: 72. / 255.,
                b: 242. / 255.,
                a: 1.,
            },
            7 => Color {
                r: 214. / 255.,
                g: 58. / 255.,
                b: 192. / 255.,
                a: 1.,
            },
            8 => Color {
                r: 230. / 255.,
                g: 52. / 255.,
                b: 185. / 255.,
                a: 1.,
            },
            _ => Color {
                r: 0.,
                g: 0.,
                b: 0.,
                a: 1.,
            },
        },
        CyclicColors::GrayScale => {
            return Color {
                r: (st * 100 % 255) as f32,
                g: (st * 100 % 255) as f32,
                b: (st * 100 % 255) as f32,
                a: 1.,
            };
        }
    }
}
