use crate::model::hero::Hero;
use crate::model::player::Player;
use crate::model::{
    ARCHER_LEGION, ARMS_PLUS, LANCER_LEGION, NAME_LIST, RIDER_LEGION, SABER_LEGION,
};
use rand;
use rand::seq::SliceRandom;
use std::cell::RefCell;

pub struct Battle {
    list: RefCell<[Player; 2]>,
    action_target_number: usize,
    action_legion_number: usize,
    defender_legion_number: usize,
    attacker_number: usize,
    defender_number: usize,
}

impl Battle {
    pub fn new(player: Player, enemy: Player) -> Self {
        Battle {
            list: RefCell::new([player, enemy]),
            action_target_number: 0,
            action_legion_number: 0,
            defender_legion_number: 0,
            attacker_number: 0,
            defender_number: 0,
        }
    }

    pub fn get_next_action_target(&mut self) -> bool {
        if self.action_target_number == 0 {
            self.action_target_number = 1;
        } else {
            self.action_target_number = 0;
            if self.action_legion_number == 3 {
                self.action_legion_number = 0
            } else {
                self.action_legion_number += 1
            }
        }
        println!(
            "action_target_number {}, action_legion_number {}",
            self.action_target_number, self.action_legion_number
        );
        true
    }

    pub fn is_action_over(&self) -> bool {
        let mut n = self.attacker_number;
        loop {
            let binding = self.list.borrow();

            let data =
                &binding.get(self.action_legion_number).unwrap().army[self.action_legion_number];
            n += 1;
            if n > data.len() - 1 {
                break;
            }
            if data[n].life > 0 {
                return false;
            }
        }
        true
    }

    pub fn state_local_fighting(&mut self) -> bool {
        let binding = self.list.borrow();
        let atk = binding.get(self.action_target_number).unwrap();
        let mut attacker = &Hero::new();
        for _i in 0..5 {
            attacker = &atk.army[self.action_legion_number][self.attacker_number];
            // println!(
            //     "action_legion_number:{} attacker_number:{} attacker {:?}",
            //     self.action_legion_number, self.attacker_number, attacker
            // );
            if attacker.life > 0 {
                break;
            }
            println!(" attacker =============");

            self.attacker_number += 1;
            if self.attacker_number == 5 {
                self.attacker_number = 0;
            }
        }

        let def = binding.get(1 - self.action_target_number).unwrap();
        let mut defender = &Hero::new();
        for _i in 0..5 {
            defender = &def.army[self.defender_legion_number][self.defender_number];
            // println!(
            //     "defender_legion_number:{} defender_number:{} defender {:?}",
            //     self.defender_legion_number, self.defender_number, defender
            // );

            if defender.life > 0 {
                break;
            }
            println!(" defender =============");

            self.defender_number += 1;
            if self.defender_number == 5 {
                self.defender_number = 0;
            }
        }

        if self.action_target_number == 0 {
            //进攻方
        }

        let arms_plus = ARMS_PLUS[self.action_legion_number][self.defender_legion_number];
        let hurt = self
            .list
            .borrow()
            .get(self.action_target_number)
            .unwrap()
            .get_hurt(arms_plus);
        let lv = self
            .list
            .borrow()
            .get(self.action_target_number)
            .unwrap()
            .level;
        let exp = self
            .list
            .borrow()
            .get(self.action_target_number)
            .unwrap()
            .get_exp(hurt as f64, lv);

        println!(
            "{} 攻击 {} 造成了 {} 点伤害 获得经验 {}  剩余:{}",
            attacker.name, defender.name, hurt, exp, defender.life
        );

        drop(binding);

        let mut binding1 = self.list.borrow_mut();

        println!(
            "defender_legion_number:{} defender_number:{}",
            self.defender_legion_number, self.defender_number
        );
        let def = &mut binding1
            .get_mut(1 - self.action_target_number)
            .unwrap()
            .army[self.defender_legion_number][self.defender_number];
        let _ = &def.get_injured(hurt);

        // defender.get_injured(hurt);

        drop(binding1);

        let all_die =
            self.is_all_die_by_legion(1 - self.action_target_number, self.defender_legion_number);

        println!("player_def all_die {:?}", all_die);
        if all_die {
            self.get_next_action_target();
        }

        true
    }

    pub fn fighting(&mut self) -> bool {
        if self.list.borrow_mut().get(0).unwrap().all_die() {
            println!(
                "进攻方 {} 获得了胜利！",
                self.list.borrow_mut().get(1).unwrap().name
            );
            return true;
        } else if self.list.borrow_mut().get(1).unwrap().all_die() {
            println!(
                "防守方 {} 获得了胜利！",
                self.list.borrow_mut().get(0).unwrap().name
            );
            return true;
        } else {
            loop {
                let binding = self.list.borrow_mut();

                let cur_hero = &binding.get(self.action_target_number).unwrap().army
                    [self.action_legion_number];
                let all_die = cur_hero.iter().all(|v| v.life <= 0);
                println!(
                    "action_target_number {}, action_legion_number {}, all_die {:?}",
                    self.action_target_number, self.action_legion_number, all_die
                );

                if !all_die {
                    break;
                }
                drop(binding);
                self.get_next_action_target();
            }

            match self.action_legion_number {
                0 => self.switch_saber_legion_target(),
                1 => self.switch_lancer_legion_target(),
                2 => self.switch_archer_legion_target(),
                3 => self.switch_rider_legion_target(),
                _ => {}
            }

            println!(
                "action_target_number {}, action_legion_number {:?}, defender_legion_number {:?}",
                self.action_target_number, self.action_legion_number, self.defender_legion_number
            );
            let binding = self.list.borrow();

            let h1 = binding.get(self.action_target_number).unwrap();
            let h2 = binding.get(1 - self.action_target_number).unwrap();
            println!(
                "{} 的 {} 向 {} 的 {} 发动攻击",
                h1.name,
                NAME_LIST[self.action_legion_number],
                h2.name,
                NAME_LIST[self.defender_legion_number]
            );
            drop(binding);
            self.state_local_fighting();
        }
        false
    }

    pub fn switch_saber_legion_target(&mut self) {
        let binding = self.list.borrow_mut();

        let h = binding.get(1 - self.action_legion_number).unwrap();
        let all_die = h.army[SABER_LEGION].iter().all(|v| v.life <= 0);
        println!("刀盾 ->目标-> 刀盾 all_die {}", all_die);

        if !all_die {
            self.defender_legion_number = SABER_LEGION;
            return;
        }
        let all_die = h.army[LANCER_LEGION].iter().all(|v| v.life <= 0);
        println!("刀盾 ->目标-> 长枪 all_die {}", all_die);
        if !all_die {
            self.defender_legion_number = LANCER_LEGION;
            return;
        }
        let all_die = h.army[ARCHER_LEGION].iter().all(|v| v.life <= 0);
        println!("刀盾 ->目标-> 弓箭 all_die {}", all_die);
        if !all_die {
            self.defender_legion_number = ARCHER_LEGION;
            return;
        }
        let all_die = h.army[RIDER_LEGION].iter().all(|v| v.life <= 0);
        println!("刀盾 ->目标-> 骑兵 all_die {}", all_die);
        if !all_die {
            self.defender_legion_number = RIDER_LEGION;
            return;
        }
        self.defender_legion_number = 999;
    }

    pub fn switch_lancer_legion_target(&mut self) {
        println!("switch_lancer_legion_target");
        let binding = self.list.borrow_mut();
        let h = binding.get(1 - self.action_legion_number).unwrap();
        let all_die = h.army[RIDER_LEGION].iter().all(|v| v.life <= 0);
        if !all_die {
            self.defender_legion_number = RIDER_LEGION;
            return;
        }
        let all_die = h.army[SABER_LEGION].iter().all(|v| v.life <= 0);
        if !all_die {
            self.defender_legion_number = SABER_LEGION;
            return;
        }
        let all_die = h.army[LANCER_LEGION].iter().all(|v| v.life <= 0);
        if !all_die {
            self.defender_legion_number = LANCER_LEGION;
            return;
        }
        let all_die = h.army[ARCHER_LEGION].iter().all(|v| v.life <= 0);
        if !all_die {
            self.defender_legion_number = ARCHER_LEGION;
            return;
        }

        self.defender_legion_number = 999;
    }
    pub fn switch_archer_legion_target(&mut self) {
        println!(
            "switch_archer_legion_target action_legion_number:{}",
            self.action_legion_number
        );

        let binding = self.list.borrow_mut();
        let h = binding.get(1 - self.action_target_number).unwrap();
        let mut rng = rand::thread_rng();
        let mut array = [SABER_LEGION, LANCER_LEGION, ARCHER_LEGION, RIDER_LEGION];
        array.shuffle(&mut rng);
        for i in array.iter() {
            let all_die = h.army[*i].iter().all(|v| v.life <= 0);
            if !all_die {
                self.defender_legion_number = *i;
                return;
            }
        }
    }
    pub fn switch_rider_legion_target(&mut self) {
        self.switch_archer_legion_target();
    }

    pub fn is_all_die_by_legion(&self, action_target_number: usize, legion_number: usize) -> bool {
        let binding = self.list.borrow();
        binding.get(action_target_number).unwrap().army[legion_number]
            .iter()
            .all(|x| x.life <= 0)
    }
}

#[cfg(test)]
mod tests {
    use crate::model::battle::Battle;
    use crate::model::player::Player;
    use crate::model::{get_random_quality, random_soldier_arr};

    #[test]
    fn fighting() {
        let (power, intelligence, command) = get_random_quality();
        let mut player = Player::new("玩家A".to_string(), power, intelligence, command);
        let soldiers = random_soldier_arr(3);
        soldiers.iter().for_each(|v| {
            println!("player soldiers {:?}", v);
            player.add_soldier((*v).clone());
        });

        player.debug();
        let (power, intelligence, command) = get_random_quality();
        let mut enemy = Player::new("玩家B".to_string(), power, intelligence, command);
        let soldiers = random_soldier_arr(3);
        for v in soldiers {
            println!("enemy soldiers {:?}", v);
            enemy.add_soldier(v);
        }
        enemy.debug();
        let mut battle = Battle::new(player, enemy);
        loop {
            if battle.fighting() {
                break;
            }
        }
    }
}
