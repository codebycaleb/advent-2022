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
                    top_3 = [current, top_3[1], top_3[2]];
                } else if current > top_3[1] {
                    top_3 = [top_3[0], current, top_3[2]];
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
        let (part_1, part_2): (u16, u16) = lines.clone().fold((0, 0), |(p1, p2), line| {
            let score1 = match line {
                "A X" => 4,
                "A Y" => 8,
                "A Z" => 3,
                "B X" => 1,
                "B Y" => 5,
                "B Z" => 9,
                "C X" => 7,
                "C Y" => 2,
                "C Z" => 6,
                _ => unimplemented!(),
            };
            let score2 = match line {
                "A X" => 3,
                "A Y" => 4,
                "A Z" => 8,
                "B X" => 1,
                "B Y" => 5,
                "B Z" => 9,
                "C X" => 2,
                "C Y" => 6,
                "C Z" => 7,
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
                let p1 = h1.chars().filter(|c| h2.contains(*c)).next().unwrap();
                (p1, line.as_bytes())
            })
            .unzip();

        let part_1 = part_1_v.iter().fold(0, |s, c| {
            let mut x = *c as u16 ^ 0x60;
            if x > 26 {
                x -= 6;
            }
            s + x
        });

        let part_2 = part_2_v
            .chunks(3)
            .map(|set| {
                set[0]
                    .iter()
                    .find(|c| set[1].contains(c) && set[2].contains(c))
                    .unwrap()
            })
            .fold(0, |s, c| {
                let mut x = *c as u16 ^ 0x60;
                if x > 26 {
                    x -= 6;
                }
                s + x
            });

        writeln!(&mut result, "Day 03, Part 1: {}", part_1).unwrap();
        writeln!(&mut result, "Day 03, Part 2: {}", part_2).unwrap();
        result
    }
}
