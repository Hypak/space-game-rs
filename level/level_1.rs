use std::collections::HashMap;

use macroquad::prelude as mq;

use crate::prelude::*;

pub struct Level1;

impl Level for Level1 {
    fn get_bases(&self) -> Vec<Base> {
        vec![
            Base::new(mq::Vec2::new(-1000.0, 0.0),
                        HashMap::from([
                               (EnemyShipType::Slow, 10),
                        ]),
                        50.0, 500.0),
            Base::new(mq::Vec2::new(-1000.0, 500.0),
                        HashMap::from([
                               (EnemyShipType::Slow, 10),
                        ]),
                        50.0, 500.0),
            Base::new(mq::Vec2::new(1000.0, -500.0),
                        HashMap::from([
                               (EnemyShipType::Slow, 10),
                        ]),
                        50.0, 500.0),
            Base::new(mq::Vec2::new(2500.0, 500.0),
                        HashMap::from([
                               (EnemyShipType::HighFriction, 10),
                        ]),
                        50.0, 1000.0),
            Base::new(mq::Vec2::new(-2500.0, 0.0),
                        HashMap::from([
                               (EnemyShipType::HighFriction, 10),
                        ]),
                        50.0, 1000.0),
            Base::new(mq::Vec2::new(500.0, 2500.0),
                        HashMap::from([
                               (EnemyShipType::HighFriction, 10),
                        ]),
                        50.0, 1000.0),
            Base::new(mq::Vec2::new(500.0, -2500.0),
                        HashMap::from([
                               (EnemyShipType::Slow, 10),
                               (EnemyShipType::Turret, 10),
                        ]),
                        50.0, 1000.0),
            Base::new(mq::Vec2::new(-1000.0, 4500.0),
                        HashMap::from([
                               (EnemyShipType::LowFriction, 10),
                               (EnemyShipType::Sniper, 10),
                        ]),
                        50.0, 1200.0),
            Base::new(mq::Vec2::new(0.0, 4500.0),
                        HashMap::from([
                               (EnemyShipType::LowFriction, 10),
                               (EnemyShipType::Sniper, 10),
                        ]),
                        50.0, 1200.0),
            Base::new(mq::Vec2::new(1000.0, 4500.0),
                        HashMap::from([
                               (EnemyShipType::LowFriction, 10),
                               (EnemyShipType::Sniper, 10),
                        ]),
                        50.0, 1200.0),
        Base::new(mq::Vec2::new(0.0, 4800.0),
                        HashMap::from([
                               (EnemyShipType::Shoot, 10),
                               (EnemyShipType::HighFriction, 10),
                        ]),
                        50.0, 1500.0),
        Base::new(mq::Vec2::new(-300.0, 4800.0),
                        HashMap::from([
                               (EnemyShipType::Slow, 10),
                               (EnemyShipType::UltraLowFriction, 10),
                        ]),
                        100.0, 1000.0),
        Base::new(mq::Vec2::new(300.0, 4800.0),
                        HashMap::from([
                               (EnemyShipType::Slow, 10),
                               (EnemyShipType::UltraLowFriction, 10),
                        ]),
                        100.0, 1000.0),
        ]
    }
    fn get_spawn_regions(&self) -> Vec<SpawnRegion> {
        let enemy_count_multiplier = 3;
        vec![
            SpawnRegion::new(HashMap::from([
                               (EnemyShipType::Slow, 9 * enemy_count_multiplier),
                               (EnemyShipType::LowFriction, 5 * enemy_count_multiplier),
                               (EnemyShipType::Turret, 2 * enemy_count_multiplier),
                        ]), 1000.0, 3000.0),
            SpawnRegion::new(HashMap::from([
                               (EnemyShipType::Slow, 36 * enemy_count_multiplier),
                               (EnemyShipType::HighFriction, 36 * enemy_count_multiplier),
                                (EnemyShipType::LowFriction, 5 * enemy_count_multiplier),
                               (EnemyShipType::Sniper, 12 * enemy_count_multiplier),
                        ]), 3000.0, 6000.0),
            SpawnRegion::new(HashMap::from([
                               (EnemyShipType::HighFriction, 36 * enemy_count_multiplier),
                                (EnemyShipType::UltraLowFriction, 5 * enemy_count_multiplier),
                               (EnemyShipType::Shoot, 144 * enemy_count_multiplier),
                        ]), 6000.0, 12000.0),
            ]
    }
}
