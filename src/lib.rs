pub mod day00 {
    use std::fmt::Write;

    pub fn run() -> String {
        let mut result: String = String::with_capacity(128);

        // let file_string = std::fs::read_to_string("data/input_day0.txt").unwrap();
        // let lines = file_string.lines();

        let part_1 = "part_1";
        let part_2 = "part_2";

        writeln!(&mut result, "Day 0, Part 1: {}", part_1).unwrap();
        writeln!(&mut result, "Day 0, Part 2: {}", part_2).unwrap();
        result
    }
}

pub mod day01 {
    use std::fmt::Write;

    pub fn run() -> String {
        let mut result: String = String::with_capacity(128);

        // Parse input, adding a new line of "END" as a signal for the processor
        let file_string = std::fs::read_to_string("data/input_day01.txt").unwrap() + "\nEND";
        let lines = file_string.lines();

        // Process the lines, keeping a `current` variable for summing groups
        // and a `top_3` array for holding the top 3 entries. Entries are tested
        // against the `top_3` when a non-number line is parsed (e.g. a blank line
        // or one that contains "END")
        let mut top_3: [u32; 3] = [0, 0, 0];
        let mut current: u32 = 0;
        for line in lines {
            if let Ok(n) = line.parse::<u32>() {
                current += n;
            } else {
                if current > top_3[0] {
                    top_3 = [current, top_3[0], top_3[1]];
                } else if current > top_3[1] {
                    top_3 = [top_3[0], current, top_3[1]];
                } else if current > top_3[2] {
                    top_3 = [top_3[0], top_3[1], current];
                }
                current = 0;
            }
        }

        writeln!(&mut result, "Day 01, Part 1: {}", top_3[0]).unwrap();
        writeln!(
            &mut result,
            "Day 01, Part 2: {}",
            top_3[0] + top_3[1] + top_3[2]
        )
        .unwrap();

        result
    }
}
pub mod day02 {
    use std::fmt::Write;

    pub fn run() -> String {
        let mut result: String = String::with_capacity(128);

        let file_string = std::fs::read_to_string("data/input_day02.txt").unwrap();
        let lines = file_string.lines();
        let (part_1, part_2): (u16, u16) = lines.fold((0, 0), |(p1, p2), line| {
            let (score1, score2) = match line {
                "A X" => (4, 3),
                "A Y" => (8, 4),
                "A Z" => (3, 8),
                "B X" => (1, 1),
                "B Y" => (5, 5),
                "B Z" => (9, 9),
                "C X" => (7, 2),
                "C Y" => (2, 6),
                "C Z" => (6, 7),
                _ => unimplemented!(),
            };
            (p1 + score1, p2 + score2)
        });

        writeln!(&mut result, "Day 02, Part 1: {}", part_1).unwrap();
        writeln!(&mut result, "Day 02, Part 2: {}", part_2).unwrap();
        result
    }
}
pub mod day03 {
    use std::fmt::Write;

    pub fn run() -> String {
        let mut result: String = String::with_capacity(128);

        let file_string = std::fs::read_to_string("data/input_day03.txt").unwrap();
        let lines = file_string.lines();

        let (part_1_v, part_2_v): (Vec<_>, Vec<_>) = lines
            .clone()
            .map(|line| {
                let (h1, h2) = line.split_at(line.len() / 2);
                let c = h1.chars().filter(|c| h2.contains(*c)).next().unwrap();
                let mut p1 = c as u16 ^ 0x60;
                if p1 > 26 {
                    p1 -= 6;
                }
                (p1, line.as_bytes())
            })
            .unzip();

        let part_1: u16 = part_1_v.iter().sum();

        let part_2: u16 = part_2_v
            .chunks(3)
            .map(|set| {
                let c = set[0]
                    .iter()
                    .find(|c| set[1].contains(c) && set[2].contains(c))
                    .unwrap();
                let mut p2 = *c as u16 ^ 0x60;
                if p2 > 26 {
                    p2 -= 6;
                }
                p2
            })
            .sum();

        writeln!(&mut result, "Day 03, Part 1: {}", part_1).unwrap();
        writeln!(&mut result, "Day 03, Part 2: {}", part_2).unwrap();
        result
    }
}
pub mod day04 {
    use std::fmt::Write;

    pub fn run() -> String {
        let mut result: String = String::with_capacity(128);

        let file_string = std::fs::read_to_string("data/input_day04.txt").unwrap();
        let lines = file_string.lines();

        let ranges = lines.map(|line| {
            let mut numbers = line
                .split(',')
                .flat_map(|pair| pair.split('-').map(|d| d.parse::<u8>().unwrap()));
            [
                numbers.next().unwrap(),
                numbers.next().unwrap(),
                numbers.next().unwrap(),
                numbers.next().unwrap(),
            ]
        });

        let part_2: Vec<_> = ranges
            .filter(|[first_min, first_max, second_min, second_max]| {
                (first_min >= second_min && first_min <= second_max)
                    || (second_min >= first_min && second_min <= first_max)
            })
            .collect();

        let part_1 = part_2
            .iter()
            .filter(|[first_min, first_max, second_min, second_max]| {
                (first_min >= second_min && first_max <= second_max)
                    || (second_min >= first_min && second_max <= first_max)
            });

        writeln!(&mut result, "Day 04, Part 1: {}", part_1.count()).unwrap();
        writeln!(&mut result, "Day 04, Part 2: {}", part_2.len()).unwrap();
        result
    }
}
pub mod day05 {
    use std::fmt::Write;

    pub fn run() -> String {
        let mut result: String = String::with_capacity(128);

        let file_string = std::fs::read_to_string("data/input_day05.txt").unwrap();
        let mut lines = file_string.lines();

        let drawing: Vec<_> = lines
            .by_ref()
            .take_while(|line| !line.starts_with(" 1"))
            .map(|line| {
                let mut bytes = line.chars();
                bytes.next();
                bytes.step_by(4).collect::<Vec<char>>()
            })
            .collect();
        lines.next(); // discard empty line
        let instructions: Vec<_> = lines.collect();

        let mut stacks_part_one: Vec<Vec<char>> = Vec::with_capacity(9);
        let num_stacks = drawing[0].len();
        for column in 0..num_stacks {
            let mut stack: Vec<char> = Vec::with_capacity(64);
            for row in (0..drawing.len()).rev() {
                let b = drawing[row][column];
                if b != ' ' {
                    stack.push(b);
                }
            }
            stacks_part_one.push(stack);
        }

        let mut stacks_part_two = stacks_part_one.clone();

        for ins in instructions {
            let mut words = ins.split(" ");
            words.next();
            let x: usize = words.next().unwrap().parse().unwrap();
            words.next();
            let y: usize = words.next().unwrap().parse().unwrap();
            words.next();
            let z: usize = words.next().unwrap().parse().unwrap();
            for _ in 1..=x {
                let v: char = stacks_part_one[y - 1].pop().unwrap();
                stacks_part_one[z - 1].push(v);
            }
            let len = stacks_part_two[y - 1].len();
            let mut slice = stacks_part_two[y - 1].drain(len - x..).collect::<Vec<_>>();
            stacks_part_two[z - 1].append(&mut slice);
        }

        let part_1 = stacks_part_one
            .iter()
            .map(|s| s[s.len() - 1])
            .collect::<String>();
        let part_2 = stacks_part_two
            .iter()
            .map(|s| s[s.len() - 1])
            .collect::<String>();

        writeln!(&mut result, "Day 05, Part 1: {}", part_1).unwrap();
        writeln!(&mut result, "Day 05, Part 2: {}", part_2).unwrap();
        result
    }
}
pub mod day06 {
    use std::fmt::Write;

    pub fn run() -> String {
        let mut result: String = String::with_capacity(128);

        let file_string = std::fs::read_to_string("data/input_day06.txt").unwrap();

        let mut windows = file_string
            .as_bytes()
            .windows(14)
            .enumerate()
            .filter(|(_i, a)| {
                a[0] != a[1]
                    && a[0] != a[2]
                    && a[0] != a[3]
                    && a[1] != a[2]
                    && a[1] != a[3]
                    && a[2] != a[3]
            });

        let part_1 = windows.next().unwrap().0 + 4;

        let part_2 = windows
            .filter(|(_i, a)| {
                for i in 0..=12 {
                    for j in (i + 1)..=13 {
                        if a[i] == a[j] {
                            return false;
                        }
                    }
                }
                true
            })
            .next()
            .unwrap()
            .0
            + 14;

        writeln!(&mut result, "Day 06, Part 1: {}", part_1).unwrap();
        writeln!(&mut result, "Day 06, Part 2: {}", part_2).unwrap();
        result
    }
}
