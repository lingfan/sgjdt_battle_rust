use crate::common::cfg::Soldier;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Hero {
    pub id: u32,
    pub name: String,
    star: u32,
    level: u32,
    power: u32,
    intelligence: u32,
    init_power: u32,
    init_intelligence: u32,
    exp_required: u32,
    pub life: u32,
    tp: u32,
}

impl Hero {
    pub fn new() -> Self {
        Hero::default()
    }

    pub fn init(&mut self, level: u32, star: u32, soldier: Soldier) {
        self.id = soldier.id;
        self.name = soldier.name;
        self.level = level;
        self.star = star;
        self.init_power = soldier.power;
        self.init_intelligence = soldier.intelligence;
        self.life = self.max_life_calculation();
    }

    pub fn set_type(&mut self, tp: u32) {
        self.tp = tp;
    }

    pub fn count_power(&self) -> u32 {
        let level = self.level as f64;

        (0.5 * self.init_power as f64 * self.star as f64 * level.sqrt() * self.init_power as f64)
            as u32
    }

    pub fn count_intelligence(&self) -> u32 {
        let level = self.level as f64;
        (self.init_intelligence as f64 * self.star as f64 * level.sqrt()) as u32
    }

    pub fn exp_calculation(&self) -> u32 {
        let level = self.level as f64;
        (100.0 * level.powf(1.5) * 100.0) as u32
    }

    pub fn max_life_calculation(&self) -> u32 {
        let level = self.level as f64;
        (5.0 * self.init_power as f64 * self.star as f64 * level.sqrt() + 25.0) as u32
    }

    pub fn soldier_value(&self) -> u32 {
        let init_power = self.init_power as f64;
        (self.star as f64 * init_power.sqrt()) as u32
    }

    pub fn get_injured(&mut self, hp: u32) {
        if self.life < hp {
            self.life = 0;
        } else {
            self.life -= hp;
        }
    }
}
