pub mod passport_processing {
    use std::collections::HashSet;
    use std::io::BufReader;
    use std::fs::File;
    use std::io::BufRead;

    pub fn part_one() -> i32 {
        let file_path = "./resources/day_four.txt";
        let required_fields: HashSet<String> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter().map(|str| str.to_string()).collect();
        let mut num_valid_passports = 0;
        let optional_field = "cid".to_string();
        let users = parse_input(file_path);

        for user in users.iter() {
            let difference: HashSet<&String> = required_fields.difference(user).collect();

            if difference.len() == 1 {
                if difference.contains(&optional_field) {
                    num_valid_passports += 1;
                }
            }  else if difference.len() == 0 {
                num_valid_passports += 1;
            }
        }
        
        num_valid_passports
    }

    pub fn part_two() -> i32 {
        0
    }

    fn parse_input(file_path: &str) -> Vec<HashSet<String>> {
        let file = File::open(file_path).expect("File does not exist");
        let reader = BufReader::new(file);
        let mut users: Vec<Vec<String>> = Vec::new();
        let mut user: Vec<String> = Vec::new();

        for line in reader.lines() {
            let input = line.unwrap();

            if input.is_empty() {
                users.push(user);
                user = Vec::new();
            } else {
                user.push(input);
            }
        }

        let result: Vec<HashSet<String>> = users.iter().map(|user| {
            let mut passport_data: HashSet<String> = HashSet::new();

            for line in user.iter() {
                let key_values: Vec<&str> = line.split(' ').collect();
                let keys = key_values.iter().map(|key_value| 
                    key_value.split(':').collect::<Vec<&str>>()[0].to_string()
                ).collect::<Vec<String>>();
                // passport_data.extend(keys.iter());
                for key in keys.iter() {
                    passport_data.insert(key.to_string());
                }
            }
            passport_data
        }).collect();

        result
    }
}