fn main() {
    day1::run("inputs/day1.txt");
}

mod utils {
    use std::fs;

    pub fn read_lines(filename: &str) -> Vec<String> {
        fs::read_to_string(filename)
            .expect(format!("day1: Unable to read file {}", filename).as_str())
            .lines()
            .map(|line: &str| String::from(line))
            .collect()
    }
}

pub mod day1 {
    use crate::utils::read_lines;
    use hashbrown::HashSet;

    pub fn run(filename: &str) -> (i32, i32) {
        let input = read_lines(filename)
            .iter()
            .map(|line: &String| line.trim().parse::<i32>().unwrap())
            .collect();

        let sum = day1_sum(&input);
        let first_cycle = day1_frequency(&input);

        println!("Day 1 : Sum is {}, first cycle is {}", sum, first_cycle);
        (sum, first_cycle)
    }

    fn day1_sum(values: &Vec<i32>) -> i32 {
        values.iter().sum()
    }

    fn day1_frequency(values: &Vec<i32>) -> i32 {
        let mut seen = HashSet::new();
        let (mut acc, mut idx) = (0, 0);
        let size = values.len();

        loop {
            acc = acc + values[idx];
            if !seen.insert(acc) {
                break;
            }
            idx = (idx + 1) % size;
        }
        acc
    }
}
