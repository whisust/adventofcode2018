fn main() {
    //    day1::run("inputs/day1.txt");
    day2::run("inputs/day2.txt");
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

pub mod day2 {
    use crate::utils::read_lines;
    use hashbrown::{HashMap, HashSet};

    struct Count {
        twos: u16,
        threes: u16,
    }

    pub fn run(filename: &str) {
        let box_ids = read_lines(filename);
        let checksum = day2_checksum(&box_ids);
        println!("Checksum is {}", checksum);

        let same_letters = day2_findboxes(&box_ids);
        println!("Common letters are {}", same_letters);
    }

    fn day2_checksum(box_ids: &Vec<String>) -> u16 {
        let mut count = Count { twos: 0, threes: 0 };
        for box_id in box_ids {
            let occurrences: HashMap<char, u32> =
                box_id.chars().fold(HashMap::new(), |mut map, ch| {
                    map.insert(ch, map.get(&ch).map(|v| v + 1).unwrap_or(1));
                    map
                });
            let counts: HashSet<&u32> = occurrences.values().collect();
            if counts.contains(&2) {
                count.twos += 1;
            };
            if counts.contains(&3) {
                count.threes += 1;
            };
        }

        count.twos * count.threes
    }

    fn day2_findboxes(box_ids: &Vec<String>) -> String {
        let mut matching: String = String::new();
        let mut ppos = 0;

        for (idx, box_id) in box_ids.iter().enumerate() {
            let others = &box_ids[(idx + 1)..];

            match find_match(box_id, others) {
                Some((_, pos)) => {
                    matching = box_id.clone();
                    ppos = pos;
                    break;
                }
                None => (),
            };
        }

        String::from(&matching[0..ppos]) + &String::from(&matching[(ppos + 1)..])
    }

    fn find_match(box_id: &String, others: &[String]) -> Option<(String, usize)> {
        let self_chars = box_id.chars();
        let mut matched = None;
        for other in others {
            let similarities: Vec<bool> = self_chars
                .clone()
                .zip(other.chars())
                .map(|(ch1, ch2)| ch1 == ch2)
                .collect();
            let diffs: Vec<&bool> = similarities.iter().filter(|b| !**b).collect();
            if diffs.len() == 1 {
                let pos = similarities.iter().position(|b| !b).unwrap();
                matched = Some((other.clone(), pos));
                break;
            }
        }
        matched
    }
}
