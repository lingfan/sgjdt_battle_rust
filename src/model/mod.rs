use crate::common::cfg::{load_soldier, Soldier};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::cmp::min;

pub mod battle;
pub mod hero;
pub mod player;

const SABER_LEGION: usize = 0;
const LANCER_LEGION: usize = 1;
const ARCHER_LEGION: usize = 2;
const RIDER_LEGION: usize = 3;

const LEGION_LIST: [usize; 4] = [SABER_LEGION, LANCER_LEGION, ARCHER_LEGION, RIDER_LEGION];

const NAME_LIST: [&str; 4] = ["刀盾兵", "长枪兵", "弓箭兵", "骑兵"];

#[derive(Serialize, Deserialize, Debug)]
enum Legion {
    SABER,
    LANCER,
    ARCHER,
    RIDER,
}
//AOE
pub const ARMS_PLUS: [[f64; 4]; 4] = [
    [0.8, 1.2, 1.5, 0.8], //saber
    [0.5, 1.0, 1.5, 2.0], //lancer
    [0.4, 1.0, 1.0, 1.0], //archer
    [0.5, 0.8, 0.8, 0.5], //rider
];

pub const RIDER_LEGION_STATE_ARRAY: [[f64; 4]; 4] = [
    [0.4, 0.6, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.3, 0.0, 0.3, 0.4],
    [0.5, 0.0, 0.0, 0.5],
];

pub const RANDOM_COEFFICIENT_DIC: [f64; 22] = [
    0.0, 0.0, 0.0, 0.5, 0.56, 0.61, 0.67, 0.72, 0.78, 0.83, 0.89, 0.94, 1.0, 1.06, 1.11, 1.17,
    1.22, 1.28, 1.33, 1.39, 1.44, 1.5,
];

pub fn get_random_value() -> [u32; 3] {
    let mut ret: [u32; 3] = [0, 0, 0];
    for i in 0..3 {
        let mut rng = rand::thread_rng();
        let random_number: f64 = rng.gen();
        ret[i as usize] = (random_number * 7.0 + 1.0) as u32;

        let _id = rand::thread_rng().gen_range(1..=100);
    }
    ret
}

//10,5,2,5,5,0,
//281,195,139,67,71,136,62,161,327,130,328,1924
//initPower,level,star
pub fn get_random_quality() -> (u32, u32, u32) {
    let mut quality_max = 15;
    let mut rng = rand::thread_rng();
    let random_number: f64 = rng.gen();
    let mut command = (random_number * 3.0) as u32;
    quality_max = quality_max - command * command;
    command += 3;

    let random_number: f64 = rng.gen();
    let mut power = (random_number * 11.0) as u32;
    quality_max = quality_max - power;
    power += 10;

    let mut intelligence = min(quality_max, 10);
    intelligence += 10;

    println!(
        "武力 {}, 智力 {}, 统帅 {}, 总 {}",
        power,
        intelligence,
        command,
        power + intelligence + command
    );
    (power, intelligence, command)
}

pub fn random_soldier_info() -> Option<Soldier> {
    let mut rng = rand::thread_rng();
    let random_number: f64 = rng.gen(); // 生成0到1之间的随机数
                                        // println!("random_number {}", random_number);

    let soldier = load_soldier("Soldier".to_string());

    if soldier.is_none() {
        return None;
    }
    let soldier = soldier.unwrap();

    let rnd_id = random_number * soldier.datas.len() as f64;
    let idx = rnd_id as usize;

    // println!("rnd_id {}", idx);

    let soldier_info = soldier.get_soldier(idx).unwrap();
    // println!("soldier_info {:?}", soldier_info);

    // rnd_id = int(Math.random() * XMLSoldier.soldier.length());
    Some(soldier_info.clone())
}

pub fn random_soldier_arr(num: usize) -> Vec<Soldier> {
    let mut ret: Vec<Soldier> = Vec::new();
    let mut max = 0;
    loop {
        if max > 999 {
            break;
        }
        max += 1;

        let info = random_soldier_info();
        if info.is_none() {
            break;
        }

        let info = info.unwrap();
        let b = ret.iter().find(|v| v.id == info.id);
        // println!("max {:?}  b {:?}", max, b);
        if b.is_some() {
            continue;
        }

        ret.push(info);
        if ret.len() >= num {
            break;
        }
    }
    // println!("max {:?}", max);

    ret
}

pub fn building_exp_calculation(lv: u32) -> u32 {
    50 * lv * lv * lv
}

pub fn exp_calculation(lv: u32) -> u32 {
    let lv = lv as f64;
    ((100.0 * lv) * lv.powf(1.3) + 1.08_f64.powf(lv)) as u32
}

pub fn random_coefficient() -> f64 {
    let random_value = get_random_value();
    let n: u32 = random_value.iter().sum();
    RANDOM_COEFFICIENT_DIC[n as usize]
}

#[cfg(test)]
mod tests {
    use crate::model::random_soldier_arr;

    #[test]
    fn it_works() {
        return;

        let arr = random_soldier_arr(10);
        println!("arr {:?}", arr);
        println!("arr len {:?}", arr.len());
        let db: sled::Db = sled::open("my_db").unwrap();

        // insert and get
        let _ = db.insert(b"yo!", b"v1");

        let aa = &db.get(b"yo!").unwrap().unwrap();
        let bb = (*aa).to_vec();
        println!("{:?}", String::from_utf8(bb));

        let aa = db.get(b"yo!");
        println!("{:?}", aa);

        let _ = db.remove(b"yo!");
        println!("{:?}", db.get(b"yo!"));

        assert_eq!(db.get(b"yo!"), Ok(None));

        let _ = db.insert(b"yo!", b"v1");
        let scan_key: &[u8] = b"a non-present key before yo!";
        let mut iter = db.range(scan_key..);
        let xx = &iter.next().unwrap().unwrap();
        println!("iter {:?}", xx);
        let (x1, x2) = xx;
        println!("iter {:?}", String::from_utf8(xx.0.to_vec()));
        println!("iter {:?}", String::from_utf8(xx.1.to_vec()));

        // assert_eq!(&iter.next().unwrap().unwrap().0, b"yo!");
        assert_eq!(iter.next(), None);

        let other_tree: sled::Tree = db.open_tree(b"cool db facts").unwrap();
        other_tree
            .insert(
                b"k1",
                &b"a Db acts like a Tree due to implementing Deref<Target = Tree>"[..],
            )
            .unwrap();
        // assert_eq!(&db.get(b"yo!").unwrap().unwrap(), b"v1");

        // 插入KV，读取Key对应的值
        let _ = db.insert("KEY1", "VAL1");
        assert_eq!(db.get(&"KEY1"), Ok(Some(sled::IVec::from("VAL1"))));

        // 范围查询
        for kv in db.range("KEY1".."KEY9") {
            println!("kv {:?}", kv);
        }

        // 删除
        let _ = db.remove(&"KEY1");

        // atomic compare and swap，可以用在并发编程中
        let _ = db.compare_and_swap("KEY1", Some("VAL1"), Some("VAL2"));

        // 阻塞直到所有修改都写入硬盘
        let _ = db.flush();
    }
}
