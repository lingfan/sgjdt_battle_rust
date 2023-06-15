use axum::{extract::Path, response::Html, Json};
// use fasthash::{spooky::Hash128, FastHash};
use crc32fast;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

#[derive(Deserialize, Serialize)]
pub struct S2CLogin {
    username: String,
    token: String,
}

pub async fn login(Path((username, password)): Path<(String, String)>) -> Json<S2CLogin> {
    let uuid = Uuid::new_v4();
    let b1 = password.as_bytes();
    let b2 = uuid.as_bytes();
    // let token = Hash128::hash(b1);
    let token = crc32fast::hash(b1);

    let ret = S2CLogin {
        token: token.to_string(),
        username: username,
    };
    Json(ret)
    // Json(vec!["foo".to_owned(), "bar".to_owned()])
}

#[derive(Serialize)]
struct S2CServerInfo {
    id: u32,
    host: String,
    port: u32,
    name: String,
}

#[derive(Serialize)]
pub struct S2CServerList {
    list: Vec<S2CServerInfo>,
    cur_server_id: u32,
}

pub async fn server_list(Path(token): Path<String>) -> Json<S2CServerList> {
    println!("{}", token);
    let mut list = Vec::new();
    list.push(S2CServerInfo {
        id: 1,
        host: "127.0.0.1".to_string(),
        port: 3001,
        name: "1服".to_string(),
    });

    let ret = S2CServerList {
        list,
        cur_server_id: 1,
    };
    Json(ret)
}

#[cfg(test)]
mod tests {
    use crate::model::{
        building_exp_calculation, exp_calculation, get_random_quality, get_random_value,
    };

    #[test]
    fn it_works() {
        for i in 1..3 {
            // get_random_quality();
            // let a = building_exp_calculation(i);
            // let b = exp_calculation(i);
            // println!("lv {:?} building {:?} player {:?}", i, a, b);
            //
            // let x = get_random_value();
            // let s: u32 = x.iter().sum();
            // println!("RANDOM_VALUE {:?} {}", x.clone(), s);
        }
    }
}
