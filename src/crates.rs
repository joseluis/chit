extern crate reqwest;

use std::io::Read;

const ENDPOINT: &str = "https://crates.io/api/v1/crates";

pub fn url(crate_name: &str) -> String {
    format!("{}/{}", ENDPOINT, crate_name)
}

pub fn owners_url(crate_name: &str) -> String {
    format!("{}/{}/owners", ENDPOINT, crate_name)
}

pub fn user_url(user_id: u64) -> String {
    let per_page = 100;
    format!(
        "{}?page=1&per_page={}&sort=alpha&user_id={}",
        ENDPOINT, per_page, user_id
    )
}

pub fn get(url: String) -> Option<serde_json::value::Value> {
    let req = reqwest::get(&url);

    match req {
        Ok(mut res) => {
            if res.status() == 200 {
                let mut res_body = String::new();
                res.read_to_string(&mut res_body)
                    .expect("fail to read crate res_body");

                Some(serde_json::from_str(&res_body).expect("fail to serde parse res_body"))
            } else {
                None
            }
        }
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    }
}
