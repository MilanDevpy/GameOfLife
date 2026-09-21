use macroquad::color::BLACK;
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
    for y in CYCLIC_PATTERN.search_distance as i32 * CYCLIC_PATTERN.search_distance as i32 * -1
        ..=CYCLIC_PATTERN.search_distance as i32
    {
        for x in CYCLIC_PATTERN.search_distance as i32 * -1
            ..=CYCLIC_PATTERN.search_distance as i32 as i32
        {
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

            if popu[target_inx as usize] == popu[ptinx(cell_pos.0, cell_pos.1) as usize] + 1 {
                score += 1;
            }
        }
    }

    let cell_who = popu[ptinx(cell_pos.0, cell_pos.1) as usize];

    if score >= CYCLIC_PATTERN.neighbours {
        if cell_who > CYCLIC_PATTERN.states - 1 {
            return 0;
        }
        return cell_who + 1;
    } else {
        return cell_who;
    }
}

pub fn state_to_color(st: u8) -> Color {
    println!("{st}");
    return Color {
        r: (st * 40) as f32,
        g: (st * 40) as f32,
        b: (st * 40) as f32,
        a: 1.,
    };
}
// pub fn state_to_color(st: u8) -> Color {
//     match st {
//         0 => Color {
//             r: 255.0,
//             g: 255.0,
//             b: 255.0,
//             a: 0.,
//         },
//         1 => Color {
//             r: 147.,
//             g: 228.,
//             b: 228.,
//             a: 1.,
//         },
//         2 => Color {
//             r: 85.,
//             g: 226.,
//             b: 168.,
//             a: 1.,
//         },
//         3 => Color {
//             r: 57.,
//             g: 236.,
//             b: 82.,
//             a: 1.,
//         },
//         4 => Color {
//             r: 193.,
//             g: 255.,
//             b: 41.,
//             a: 1.,
//         },
//         5 => Color {
//             r: 218.,
//             g: 203.,
//             b: 12.,
//             a: 1.,
//         },
//         _ => {
//             assert!(true);
//             BLACK
//         }
//     }
// }
