use std::{collections::HashMap, fs};

fn read_users_file() -> String {
    let data = match fs::read_to_string("users.json") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            String::new()
        }
    };
    data
}

pub fn get_users() -> HashMap<String, HashMap<String, i64>> {
    let users =
        serde_json::from_str::<HashMap<String, HashMap<String, i64>>>(&read_users_file()).unwrap();
    users
}

pub fn set_users(users: HashMap<String, HashMap<String, i64>>) {
    let data = serde_json::to_string(&users).unwrap();
    fs::write("users.json", data).expect("Unable to write file");
}