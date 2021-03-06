use std::env::args;

mod problems;
mod utils;

use problems::day_one::report_repair;
use problems::day_two::password_philosophy;
use problems::day_three::toboggan_trajectory;
use problems::day_four::passport_processing;
use problems::day_five::binary_boarding;
use problems::day_six::custom_customs;

fn main() {
    let args: Vec<String> = args().collect();

    let question_day = &args[1].parse::<i32>().unwrap();

    match question_day {
        1 => {
            let (num1, num2) = report_repair::two_sum(2020);
            println!("Part 1: ");
            println!("{}", num1 * num2);
            let (num1, num2, num3) = report_repair::three_sum(2020);
            println!("Part 2: ");
            println!("{}", num1*num2*num3);
        }
        2 => {
            let result = password_philosophy::part_one();
            println!("Part 1: ");
            println!("{}", result);
            let result_2 = password_philosophy::part_two();
            println!("Part 2: ");
            println!("{}", result_2);
        }
        3 => {
            let result = toboggan_trajectory::part_one();
            println!("Part 1: ");
            println!("{}", result);
            let result_2 = toboggan_trajectory::part_two();
            println!("Part 2: ");
            println!("{}", result_2);
        }
        4 => {
            let result = passport_processing::part_one();
            println!("Part 1: ");
            println!("{}", result);
            let result_2 = passport_processing::part_two();
            println!("Part 2: ");
            println!("{}", result_2);
        }
        5 => {
            let result = binary_boarding::part_one();
            println!("Part 1: ");
            println!("{}", result);
            let result = binary_boarding::part_two();
            println!("Part 2: ");
            println!("{}", result);
        }
        6 => {
            let result = custom_customs::part_one();
            println!("Part 1: ");
            println!("{}", result);
            let result = custom_customs::part_two();
            println!("Part 2: ");
            println!("{}", result);
        }
        _ => println!("Question has no been finished yet")
    }
}
