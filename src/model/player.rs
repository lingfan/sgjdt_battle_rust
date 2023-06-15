use crate::common::cfg::Soldier;
use crate::model::hero::Hero;
use crate::model::random_coefficient;
use serde::{Deserialize, Serialize};
use std::cmp::min;

use super::LEGION_LIST;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Player {
    pub name: String,
    pub init_power: u32,
    init_intelligence: u32,
    init_command: u32,
    power: u32,        //武力
    intelligence: u32, //智力
    command: u32,      //统帅
    pub level: u32,    //等级
    exp_required: u32, //exp
    gold: u32,         //黄金
    food: u32,         //食物
    wood: u32,         //木材
    city_level: u32,   //主城
    smithy_level: u32, //铁匠
    farm_level: u32,   //农场
    design_name: u32,  //关卡名
    design_enemy: u32, //敌人
    pub army: [[Hero; 5]; 4],
    rider_legion_state: u32,
    dash_power: u32,

    saber_legion: [Hero; 5],
    lancer_legion: [Hero; 5],
    archer_legion: [Hero; 5],
    rider_legion: [Hero; 5],
}

impl Player {
    pub fn new(name: String, init_power: u32, init_intelligence: u32, init_command: u32) -> Self {
        let mut player = Player::default();
        player.name = name;
        player.init_power = init_power;
        player.init_intelligence = init_intelligence;
        player.init_command = init_command;
        player.init();
        player.re_calc();
        player
    }

    pub fn init(&mut self) {
        self.level = 1;
    }
    pub fn re_calc(&mut self) {
        let a = self.level as f64;
        let c = (self.level - 1) as f64;
        self.power = (self.init_power as f64 * a.sqrt() + 8_f64 * c) as u32;
        self.intelligence = (self.init_intelligence as f64 * a.sqrt() + 8_f64 * c) as u32;
        self.command = (0.9_f64 * self.init_command as f64 * a.sqrt() * 2_f64 * c) as u32;
    }

    pub fn add_soldier(&mut self, soldier: Soldier) {
        let tp = soldier.r#type as usize;
        for i in 0..=4 {
            let b = &self.army[tp][i as usize];
            // println!(" b {:?} {:?} {:?}", i, b, (&soldier).clone());

            if b.id == 0 {
                let mut hero = Hero::new();
                hero.init(1, 1, (&soldier).clone());
                self.army[tp][i as usize] = hero;
                break;
            }
        }
    }

    pub fn debug(&self) {
        // println!("debug {:?}", self.army);
    }

    pub fn all_die(&self) -> bool {
        for i in LEGION_LIST.iter() {
            let die = self.army[*i].iter().all(|x| x.life <= 0);
            // println!("all_die {} {}", i, die);
            if !die {
                return false;
            }
        }
        true
    }

    pub fn get_hurt(&self, arms_plus: f64) -> u32 {
        let a = self.power as f64;
        let b = random_coefficient();
        let c = 1.05_f64.powf(self.init_power as f64 - 15_f64);
        println!("a:{}, arms_plus:{}, b:{:?} c:{:?}", a, arms_plus, b, c);

        (a * arms_plus * b * c) as u32
    }

    pub fn get_exp(&self, hurt: f64, player_level: u32) -> u32 {
        let b = min(10, player_level - self.level);
        let a = 1.4_f64.powf(b as f64);
        return min(20000_u32, (hurt * 0.1 * a) as u32);
    }

    pub fn get_hero(&mut self, legion_number: usize, number: usize) -> Option<&mut Hero> {
        if let Some(legion) = self.army.get_mut(legion_number) {
            if let Some(hero) = legion.get_mut(number) {
                return Some(hero);
            }
        }
        None
    }
}
