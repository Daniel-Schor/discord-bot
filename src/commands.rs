use std::collections::HashMap;

pub fn get_commands() -> HashMap<String, String> {
    let mut commands = HashMap::from([
        (
            "!w2g".to_string(),
            "https://w2g.tv/?r=lg383dt10ofhepndm5".to_string(),
        ),
        ("!test".to_string(), "This is a test".to_string()),
    ]);

    let mut joined_keys = String::from("Other commands:\n");

    for key in commands.keys() {
        joined_keys.push_str("- ");
        joined_keys.push_str(key);
        joined_keys.push_str("\n");
    }

    commands.insert("!help".to_string(), joined_keys);

    commands
}
