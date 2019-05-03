#[macro_use]
extern crate lazy_static;

fn main() {
    //    day1::run("inputs/day1.txt");
    //    day2::run("inputs/day2.txt");
    day3::run("inputs/day3.txt");
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
        println!("Day 2, part 1 : Checksum is {}", checksum);

        let same_letters = day2_findboxes(&box_ids);
        println!("Day 2, part 2 : Common letters are {}", same_letters);
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

pub mod day3 {
    extern crate regex;

    use crate::utils::read_lines;
    use hashbrown::{HashMap, HashSet};
    use regex::Regex;

    pub fn run(filename: &str) {
        let claims = read_lines(&filename).iter().map(Claim::parse).collect();

        let sq_inches = day3_find_overlapping(&claims);
        println!("Day 3, part 1 : square inches overlapping = {}", sq_inches);

        let not_overlapping = day3_find_not_overlapping(&claims);
        println!(
            "Day 3, part 2 : non overlapping claim is #{}",
            not_overlapping.id
        );
    }

    lazy_static! {
        static ref RE: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    }

    #[derive(Debug, Eq, Hash, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Point {
        fn new(x: i32, y: i32) -> Point {
            Point { x, y }
        }
    }

    #[derive(Debug, Eq, Hash, PartialEq)]
    struct Claim {
        id: i32,
        padding_left: i32,
        padding_top: i32,
        width: i32,
        height: i32,
    }

    impl Claim {
        fn new() -> Claim {
            Claim {
                id: 0,
                padding_left: 0,
                padding_top: 0,
                width: 0,
                height: 0,
            }
        }

        fn parse(s: &String) -> Claim {
            let to_int = |l: &str| l.parse::<i32>().unwrap();
            let mut c = Claim::new();
            for cap in RE.captures_iter(s) {
                c = Claim {
                    id: to_int(&cap[1]),
                    padding_left: to_int(&cap[2]),
                    padding_top: to_int(&cap[3]),
                    width: to_int(&cap[4]),
                    height: to_int(&cap[5]),
                }
            }
            c
        }

        fn positions(&self) -> Vec<(i32, i32)> {
            let mut poses = vec![];
            for i in (self.padding_left + 1)..(self.padding_left + self.width + 1) {
                for j in (self.padding_top + 1)..(self.padding_top + self.height + 1) {
                    poses.push((i, j));
                }
            }
            poses
        }

        fn corners(&self) -> (Point, Point) {
            let top_right = Point::new(self.padding_left + self.width, self.padding_top + 1);
            let bottom_left = Point::new(self.padding_left + 1, self.padding_top + self.height);
            (top_right, bottom_left)
        }

        fn overlaps(&self, other: &Claim) -> bool {
            let (tr1, bl1) = self.corners();
            let (tr2, bl2) = other.corners();

            !((tr1.y > bl2.y || bl1.y < tr2.y) || (tr1.x < bl2.x || bl1.x > tr2.x))
        }
    }

    fn day3_find_overlapping(claims: &Vec<Claim>) -> usize {
        let mut pos_map = HashMap::new();
        let poses: Vec<(i32, i32)> = claims.iter().flat_map(|claim| claim.positions()).collect();

        for pos in &poses {
            pos_map.insert(
                *pos,
                pos_map.get(pos).map(|c| c + 1 as i32).unwrap_or(1 as i32),
            );
        }

        let v: Vec<&i32> = pos_map.values().filter(|i| **i > 1).collect();
        v.len()
    }

    fn day3_find_not_overlapping(claims: &Vec<Claim>) -> &Claim {
        let mut the_one = None;
        let mut already_visited = HashSet::new();

        for claim in claims {
            if already_visited.contains(&claim.id) {
                continue;
            }
            let mut overlapped = false;
            for c in claims {
                if c.id != claim.id && claim.overlaps(c) {
                    already_visited.insert(&claim.id);
                    already_visited.insert(&c.id);
                    overlapped = true;
                    break;
                }
            }
            if !overlapped {
                the_one = Some(claim);
                break;
            }
        }

        the_one.unwrap()
    }

    #[test]
    fn test_claim_positions() {
        let claim = Claim {
            id: 0,
            padding_left: 3,
            padding_top: 2,
            width: 3,
            height: 4,
        };

        let poses = claim.positions();
        let expected = vec![
            (4, 3),
            (4, 4),
            (4, 5),
            (4, 6),
            (5, 3),
            (5, 4),
            (5, 5),
            (5, 6),
            (6, 3),
            (6, 4),
            (6, 5),
            (6, 6),
        ];
        assert_eq!(&expected, &poses);
    }

    #[test]
    fn test_claim_overlaps() {
        let c1 = Claim::parse(&"#1 @ 1,3: 4x4".to_string());
        let c2 = Claim::parse(&"#2 @ 3,1: 4x4".to_string());
        let c3 = Claim::parse(&"#3 @ 5,5: 2x2".to_string());

        assert!(c1.overlaps(&c2));
        assert!(c2.overlaps(&c1));
        assert!(!c1.overlaps(&c3));
        assert!(!c2.overlaps(&c3));
        assert!(c1.overlaps(&c1));
    }

    #[test]
    fn test_claim_corners() {
        let c1 = Claim::parse(&"#1 @ 1,3: 4x4".to_string());
        let c2 = Claim::parse(&"#2 @ 3,1: 4x4".to_string());
        let (tr, bl) = c1.corners();
        let (tr2, bl2) = c2.corners();

        assert_eq!(Point::new(5, 4), tr);
        assert_eq!(Point::new(2, 7), bl);

        assert_eq!(Point::new(7, 2), tr2);
        assert_eq!(Point::new(4, 5), bl2);
    }
}
