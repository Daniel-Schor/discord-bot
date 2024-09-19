use std::collections::HashMap;

pub fn get_commands() -> HashMap<String, String> {
    let mut commands: HashMap<String, String> = HashMap::new();

    // test command
    commands.insert("!test".to_string(), "MOIN".to_string());

    // watch2gether link
    commands.insert(
        "!w2g".to_string(),
        "https://w2g.tv/?r=lg383dt10ofhepndm5".to_string(),
    );

    // help command
    let mut joined_keys = String::from("
                    Hello there, Human!\n\
                    You have summoned me. Let's see about getting you what you need.\n\
                    ❓ Need technical help?\n\
                    ➡️ Post in the <#1286338171642314886> channel and other humans will assist you.\n\
                    ❓ Looking for the Code of Conduct?\n\
                    ➡️ Here it is: <https://opensource.facebook.com/code-of-conduct>\n\
                    ❓ Something wrong?\n\
                    ➡️ You can flag an admin with @admin\n\
                    ❓ You want to know other commands?\n\
                    ");
    for key in commands.keys() {
        joined_keys.push_str("➡️");
        joined_keys.push_str(key);
        joined_keys.push_str("\n");
    }
    joined_keys.push_str(
        "I hope that resolves your issue!\n\
        — HelpBot 🤖\n",
    );

    commands.insert("!help".to_string(), joined_keys);

    commands
}
