use rand::random_range;

use crate::config::*;
use crate::ptinx;
pub fn init_pop() -> Vec<CellState> {
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
pub fn scan_cell(cell_pos: (u32, u32), popu: &[CellState]) -> CellState {
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
