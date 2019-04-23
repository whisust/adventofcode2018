fn main() {
    day1::run("inputs/day1.txt");
}

pub mod day1 {
    use hashbrown::HashSet;
    use std::fs;

    pub fn run(filename: &str) -> (i32, i32) {
        let input = fs::read_to_string(filename)
            .expect(format!("day1: Unable to read file {}", filename).as_str());

        let int_vec = input
            .split("\n")
            .map(|line: &str| String::from(line).trim().parse::<i32>().unwrap())
            .collect();

        let sum = day1_sum(&int_vec);
        let first_cycle = day1_frequency(&int_vec);

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
