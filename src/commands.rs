use std::{collections::HashMap, fs};

fn get_users() -> HashMap<String, HashMap<String, u64>> {
    let data = match fs::read_to_string("users.json") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            String::new()
        }
    };

    serde_json::from_str::<HashMap<String, HashMap<String, u64>>>(&data).unwrap()
}

pub fn get_commands() -> HashMap<String, String> {
    let mut commands = HashMap::from([
        (
            "!w2g".to_string(),
            "https://w2g.tv/?r=lg383dt10ofhepndm5".to_string(),
        ),
        ("!test".to_string(), "This is a test".to_string()),
    ]);

    // TODO update when called
    // building user stats
    let users = get_users();
    let mut users_stats = String::from("User time spent in voice channels:\n");
    for key in users.keys() {
        users_stats.push_str(
            format!(
                "<@{}>: {}s",
                key,
                users.get(&key.to_string()).unwrap().get("duration").unwrap()
            )
            .trim(),
        );
        users_stats.push_str("\n");
    }
    commands.insert("!users".to_string(), users_stats);

    // building commands list
    let mut joined_keys = String::from("Other commands:\n");

    for key in commands.keys() {
        joined_keys.push_str("- ");
        joined_keys.push_str(key);
        joined_keys.push_str("\n");
    }

    commands.insert("!help".to_string(), joined_keys);

    commands
}
