pub mod custom_customs {
    use std::{collections::HashSet};

    use crate::utils::file_utils;

    pub fn part_one() -> usize {
        let file_path = "./resources/day_six.txt";
        let custom_forms = file_utils::read_lines(file_path);
        let mut declaration_count = 0;
        let mut declaration_group: HashSet<char> = HashSet::new();

        for form in custom_forms.iter() {
            if form == "" {
                declaration_count += declaration_group.len();
                declaration_group.clear();
            } else {
                declaration_group.extend(form.chars());
            }
        }
        declaration_count + declaration_group.len()
    }

    pub fn part_two() -> usize {
        let file_path = "./resources/day_six.txt";
        let custom_forms = file_utils::read_lines(file_path);
        let mut declaration_count = 0;

        let mut declaration_groups: Vec<Vec<&str>> = Vec::new();
        let mut declaration_group: Vec<&str> = Vec::new();

        for form in custom_forms.iter() {  
            if form.is_empty() {
                declaration_groups.push(declaration_group.clone());
                declaration_group.clear();
            } else {
                declaration_group.push(form);
            }
        }

        declaration_groups.push(declaration_group.clone());     

        for (_index, declaration_group) in declaration_groups.iter().enumerate() {
            let mut form = HashSet::<char>::new();
            let n = declaration_group.len();
            form.extend(declaration_group[0].chars());

            for person in declaration_group[1..n].iter() {
                form = form.intersection(&person.chars().collect::<HashSet<char>>()).map(|chr| chr.clone()).collect();
            }
            
            declaration_count += form.len();
        };

        println!("{:?}", declaration_groups[declaration_groups.len()-1]);

        declaration_count
    }
}