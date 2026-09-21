use macroquad::color::*;

pub const SCREEN_SIZE: (u32, u32) = (700, 700); // How big will be the simulation, pixels are cells (x,y)
pub const BG_COLOR: Color = BLACK; // Background Color
pub const FULL_SCREEN: bool = false; // Is full screen (tbh it's usefull only if you put the screensize to your screen size)
pub const WAIT_FOR_SIGNAL: bool = true; // If true, the simulation will wait you to press the space button

pub const EXPORTING: bool = false; // Do you want to export the sim to a gif
pub const EXPORTING_RATE: u16 = 5; // will export a frame every EXPORTING_RATE generations
pub const SCREEN_LIMIT: u16 = 200; // Maximul number of frames the gif will contain
pub const DELAY: u8 = 20; // Time between each frame
pub const EXPORT_DIR: &str = "/home/cookie/Documents/Dev/Rust/Bordel Evolutif/gof_v2/Frames/frame"; // Put a trash dir

// Cyclic Cellular Automata Sim

pub enum SearchPattern {
    Cross,
    Classic,
}
pub struct CyclicPattern {
    pub pattern: SearchPattern,
    pub search_distance: u8,
    pub states: u8,
    pub neighbours: u8,
}
pub const CYCLIC_PATTERN: CyclicPattern = CyclicPattern {
    pattern: SearchPattern::Classic,
    search_distance: 1,
    states: 5,
    neighbours: 4,
};
// Life Sim
pub const SPAWN_TYPE: SpawnType = SpawnType::FullCircle; // Spawn in a circle (cooler) or randomly
pub const SPAWN_RATE: u8 = 8; // If FullRandom, every cell will have 1/SPAWN_RATE chance to spawn
pub const CELL_TYPE: CellType = CellType {
    b: &[1],
    s: &[1],
    color: WHITE,
};
// Cells Spawn Parameters
pub const CIRCLE_ROUND: u32 = 70; // If FullCircle, the exterior circle
pub const CIRCLE_WEIGHT: u32 = 20; // If Fullcircle, the interior circle (will fill with Alive cells between CIRCLE_ROUND & CIRCLE_WEIGHT)
pub enum SpawnType {
    FullRandom,
    FullCircle,
}
pub const SIMULATION_TYPE: SimulationType = SimulationType::Life;
#[derive(PartialEq)]
pub enum SimulationType {
    Life,
    Cyclic,
}
#[derive(PartialEq, Copy, Clone)]
pub enum CellState {
    Alive,
    Dead,
}

pub struct CellType {
    pub b: &'static [u8],
    pub s: &'static [u8],
    pub color: Color,
}
impl CellType {
    pub fn check_surviving(&self, actual_state: CellState, score: u8) -> CellState {
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
