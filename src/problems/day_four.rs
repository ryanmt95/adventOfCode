pub mod passport_processing {
    use std::collections::{HashMap, HashSet};
    use std::io::BufReader;
    use std::fs::File;
    use std::io::BufRead;
    use regex::Regex;

    pub fn part_one() -> i32 {
        let file_path = "./resources/day_four.txt";
        let required_fields: HashSet<&str> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter().cloned().collect();
        let mut num_valid_passports = 0;
        let user_inputs = parse_input(file_path);

        let users: Vec<HashSet<&str>> = user_inputs.iter().map(|user| {
            let mut passport_data: HashSet<&str> = HashSet::new();

            for line in user.iter() {
                let key_values: Vec<&str> = line.split(' ').collect();
                let keys = key_values.iter().map(|key_value| 
                    key_value.split(':').collect::<Vec<&str>>()[0]
                ).collect::<Vec<&str>>();
                for key in keys.iter() {
                    passport_data.insert(key);
                }
            }
            passport_data
        }).collect();


        for user in users.iter() {
            let difference: HashSet<&str> = required_fields.difference(&user).cloned().collect();

            if difference.len() == 0 {
                num_valid_passports += 1;
            }
        }
        
        num_valid_passports
    }

    pub fn part_two() -> i32 {
        let file_path = "./resources/day_four.txt";
        let required_fields: HashSet<&str> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter().cloned().collect();
        let mut num_valid_passports = 0;
        let user_inputs = parse_input(file_path);

        let users: Vec<HashMap<&str, &str>> = user_inputs.iter().map(|user| {
            let mut passport_data: HashMap<&str, &str> = HashMap::new();

            for line in user.iter() {
                let key_values: Vec<&str> = line.split(' ').collect();
                for key_value in key_values.iter() {
                    let key_value_list = key_value.split(":").collect::<Vec<&str>>();

                    passport_data.insert(key_value_list[0], key_value_list[1]);
                }
            }
            passport_data
        }).collect();

        for user in users.iter() {
            // hashmap.keys() returns Iterator<Item = &'a K>
            // .cloned() transforms that to Iterator<Item = K>
            // .collect builds a collection from that
            let keys: HashSet<&str> = user.keys().cloned().collect();
            let difference: HashSet<&str> = required_fields.difference(&keys).cloned().collect();

            if difference.len() == 0 {
                if validate_field(user, &required_fields) {
                    num_valid_passports += 1
                };
            }
        }

        num_valid_passports
    }

    fn validate_field(user: &HashMap<&str, &str>, required_fields: &HashSet<&str>) -> bool {
        let keys: HashSet<&str> = user.keys().cloned().collect();
        let difference: HashSet<&str> = required_fields.difference(&keys).cloned().collect();

        if difference.len() != 0 {
            return false;
        }
        
        for (key, value) in user {
            if validate_field_value(key, value) == false {
                return false
            }
        }

        true
    }

    fn validate_field_value(key: &str, value: &str) -> bool {
        match key {
            "byr" => {
                let val = value.parse::<i32>().unwrap();
                if val >= 1920 && val <= 2002 {
                    return true;
                }
            }
            "iyr" => {
                let val = value.parse::<i32>().unwrap();
                if val >= 2010 && val <= 2020 {
                    return true;
                }
            }
            "eyr" => {
                let val = value.parse::<i32>().unwrap();
                if val >= 2020 && val <= 2030 {
                    return true;
                }
            }
            "hgt" => {
                let n = value.len();
                let h_type = &value[n-2..n];
                match h_type {
                    "cm" => {
                        let height = value[0..n-2].parse::<i32>().unwrap();
                        if height >= 150 && height <= 193 {
                            return true
                        }
                    }
                    "in" => {
                        let height = value[0..n-2].parse::<i32>().unwrap();
                        if height >= 59 && height <= 76 {
                            return true
                        }
                    }
                    _ => {
                        println!("{}", &value);
                        println!("Invalid height type");
                        return false
                    }
                }
            }
            "hcl" => {
                let re = Regex::new(r"#([0-9]|[a-f]){6}").unwrap();
                return re.is_match(value)
            }
            "ecl" => {
                let valid_eye_entry: HashSet<&str> = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().cloned().collect();
                return valid_eye_entry.contains(value) 
            }
            "pid" => {
                let re = Regex::new(r"([0-9]){9}").unwrap();
                if re.is_match(value) && value.len() == 9 {
                    return true
                }
            }
            "cid" => {
                return true
            }
            _ => {
                println!("Invalid key");
            }
        }
        false
    }

    fn parse_input(file_path: &str) -> Vec<Vec<String>> {
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

        users
    }
}