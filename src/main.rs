use std::env::args;

mod problems;
mod utils;

use problems::day_one::report_repair;
use problems::day_two::password_philosophy;

fn main() {
    let args: Vec<String> = args().collect();

    // ::<> is the turbofish operator
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
        _ => println!("Question has no been finished yet")
    }
}