pub mod level_1;

use crate::prelude::*;

pub trait Level {
    fn get_bases(&self) -> Vec<Base>;
    fn get_spawn_regions(&self) -> Vec<SpawnRegion>;
}
