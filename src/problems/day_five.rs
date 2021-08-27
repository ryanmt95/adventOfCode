pub mod binary_boarding {
    use crate::utils::file_utils;
    use std::cmp;

    pub fn part_one() -> i32 {
        let file_path = "./resources/day_five.txt";
        let boarding_passes = file_utils::read_lines(file_path);
        let mut top_seat_id = 0;

        for boarding_pass in boarding_passes.iter() {
            let rows = (0, 128);
            let row_num = decrypt_row(&boarding_pass[0..7], rows.0, rows.1);
            let cols = (0, 8);
            let col_num = decrypt_col(&boarding_pass[7..10], cols.0, cols.1);

            let seat_id = row_num * 8 + col_num;
            top_seat_id = cmp::max(seat_id, top_seat_id);
        }

        top_seat_id
    }

    pub fn part_two() -> i32 {
        let file_path = "./resources/day_five.txt";
        let boarding_passes = file_utils::read_lines(file_path);
        let mut seat_ids = Vec::new();

        for boarding_pass in boarding_passes.iter() {
            let rows = (0, 128);
            let row_num = decrypt_row(&boarding_pass[0..7], rows.0, rows.1);
            let cols = (0, 8);
            let col_num = decrypt_col(&boarding_pass[7..10], cols.0, cols.1);

            let seat_id = row_num * 8 + col_num;
            seat_ids.push(seat_id);
        }

        seat_ids.sort();

        for (index, seat_id) in seat_ids[0..seat_ids.len()-1].iter().enumerate() {
            if seat_id + 1 != seat_ids[index+1] {
                return seat_id + 1;
            }
        }

        0
    }

    fn decrypt_row(pass: &str, mut start: i32, mut end: i32) -> i32 {
        for char in pass.chars() {
            match char {
                'F' => {
                    end = (start + end)/2
                }
                'B' => {
                    start = (start + end)/2
                }
                _ => {
                    println!("Invalid input for decrypt row");
                }
            }
        }
        
        start
    }

    fn decrypt_col(pass: &str, mut start: i32, mut end: i32) -> i32 {
        for char in pass.chars() {
            match char {
                'L' => {
                    end = (start + end)/2
                }
                'R' => {
                    start = (start + end)/2
                }
                _ => {
                    println!("Invalid input for decrypt col");
                }
            }
        }
        
        start
    }
}