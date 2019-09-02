pub mod company {
    use std::collections::HashMap;
    pub fn parse_command(command: &str, departments: &mut HashMap<String, Vec<String>>) {
        let words: Vec<&str> = command.split_whitespace().collect();
        let mut response = "".to_string();
        if words[0].to_lowercase() == "add" {
            departments
                .entry(words[3].to_string())
                .or_insert(Vec::new())
                .push(words[1].to_string());
            response = format!("Added {} to {}", words[1], words[3]);
        } else if words[0].to_lowercase() == "get" {
            let people_dpt = departments.get(words[1]);
            match people_dpt {
                Some(people_dpt) => {
                    for person in people_dpt {
                        response.push_str(&format!("{}\n", person));
                    }
                }
                None => println!("No department {}", words[1]),
            }
        } else {
            response = format!("No command found for: {}", command)
        }
        println!("{}", &response);
    }
}
