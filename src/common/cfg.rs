use log::error;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Soldier {
    pub id: u32,
    pub name: String,
    pub power: u32,
    pub intelligence: u32,
    pub maxStar: u32,
    pub r#type: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Datas<T> {
    pub datas: Vec<T>,
}

impl Datas<Soldier> {
    pub fn get_soldier(&self, id: usize) -> Option<&Soldier> {
        // self.datas.iter().find(|&s| s.id == id)
        self.datas.get(id)
    }
}

pub fn load_soldier(name: String) -> Option<Datas<Soldier>> {
    let path = std::env::current_dir();
    if let Err(e) = path {
        error!("{:?}", e);
        return None;
    }
    let path = path.unwrap();
    let str = path.as_os_str().to_str();
    if let None = str {
        error!("reload_temps can not path to_str!");
        return None;
    }
    let str = str.unwrap();
    let res = format!("{}/cfg/{}.json", str.to_string(), name);
    let data = std::fs::read_to_string(res).unwrap();
    // println!("{}", data);
    let ret = serde_json::from_str::<Datas<Soldier>>(&data);
    if let Err(e) = ret {
        error!("{:?}", e);
        return None;
    }

    ret.ok()
}

#[cfg(test)]
mod tests {
    use crate::common::cfg::{load_soldier, Soldier};

    #[test]
    fn it_works() {
        let mut b = Vec::new();
        b.push(Soldier {
            id: 1,
            name: "String".to_string(),
            power: 1,
            intelligence: 1,
            maxStar: 1,
            r#type: 1,
        });

        // let ret: SoldierList = serde_json::from_str(&c).unwrap();
        // println!("{:?}", ret);

        // let a = load_soldier("Soldier".to_string());
        // println!("{:?}", a);
    }
}
