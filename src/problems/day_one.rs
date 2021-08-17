pub mod report_repair {
    use crate::utils::file_utils;
    use std::collections::HashMap;

    pub fn two_sum(sum: i32) -> (i32, i32) {
        let file_path = "./resources/day_one.txt";
        let numbers = file_utils::read_numbers(file_path);

        //print a list of numbers
        // println!("{:?}", numbers);

        let mut num_map: HashMap<i32, bool> = HashMap::new();

        for num in numbers.iter() {
            if num_map.contains_key(&(sum-num)) {
                return (*num, sum-num);
            } else {
                num_map.insert(*num, true);
            }
        }

        return (0, 0);
    }

    pub fn three_sum(sum: i32) -> (i32, i32, i32) {
        let file_path = "./resources/day_one.txt";
        let numbers = file_utils::read_numbers(file_path);

        for num in numbers.iter() {
            let (num2, num3) = two_sum(sum-*num);

            if num2 != 0 {
                return (*num, num2, num3);
            }
        }
        return (0, 0, 0);
    }
}