pub mod toboggan_trajectory {
    use crate::utils::file_utils;

    // this implements the debug trait on the struct
    // allows you to print with {:?}
    #[derive(Debug)]
    struct Point {
        x: usize,
        y: usize,
    }

    pub fn part_one() -> i32 {
        counting_trees(3, 1)
    }

    pub fn part_two() -> i64 {
        let slopes = vec!{(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)};
        let mut num_trees: i64 = 1;
        
        for slope in slopes.iter() {
            num_trees *= counting_trees(slope.0, slope.1) as i64;
        }

        num_trees
    }

    fn counting_trees(add_x: usize, add_y: usize) -> i32 {
        let file_path = "./resources/day_three.txt";
        let map = file_utils::read_lines(file_path);

        let mut num_trees = 0;
        let mut location = Point{ x: 0, y: 0};
        let y_max = map.len();
        let x_max = map[0].len();

        location.y += add_y;
        location.x = (location.x + add_x) % x_max;

        while location.y < y_max {      
            let chr = map[location.y].chars().nth(location.x).unwrap();
            if chr == '#' {
                num_trees += 1;
            }

            location.y += add_y;
            location.x = (location.x + add_x) % x_max;
        }

        num_trees
    }
}