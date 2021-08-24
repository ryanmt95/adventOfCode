pub mod password_philosophy {

    // this implements the debug trait on the struct
    // allows you to print with {:?}
    #[derive(Debug)]
    struct PasswordConfig<'a> {
        min: i32,
        max: i32,
        chr: char,
        password: &'a str
    }

    use crate::utils::file_utils;

    pub fn part_one() -> i32 {
        let mut total_count = 0;
        let file_path = "./resources/day_two.txt";
        let lines = file_utils::read_lines(file_path);

        // let result = parse_password(&lines[0]);
        // println!("{:?}", result);

        for line in lines.iter() {
            let password_config = parse_password(line);
            let PasswordConfig { min, max, chr, password} = password_config;
            let count = password.matches(chr).count();

            if (min..max+1).contains(&(count as i32)) {
                total_count += 1
            }
        }
        
        total_count
    }

    pub fn part_two() -> i32 {
        let mut total_count = 0;
        let file_path = "./resources/day_two.txt";
        let lines = file_utils::read_lines(file_path);

        for line in lines.iter() {
            let mut valid_password = 0;
            let password_config = parse_password(line);
            let PasswordConfig { min, max, chr, password} = password_config;
            if password.chars().nth((min as usize) - 1).unwrap() == chr {
                valid_password += 1;
            }
            if password.chars().nth((max as usize) - 1).unwrap() == chr {
                valid_password += 1;
            }

            if valid_password == 1 {
                total_count += 1;
            }
        }

        total_count
    }

    fn parse_password(input: &str) -> PasswordConfig {
        let vec: Vec<&str> = input.split(" ").collect();

        let min_max: Vec<&str> = vec[0].split("-").collect();
        PasswordConfig {
            min: min_max[0].parse::<i32>().unwrap(),
            max: min_max[1].parse::<i32>().unwrap(),
            chr: vec[1].chars().next().unwrap(),
            password: vec[2]
        }
    }
}