use rand::random_range;

use crate::config::*;

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
