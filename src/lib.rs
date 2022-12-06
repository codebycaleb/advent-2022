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
        let mut top_3: [i32; 3] = [0, 0, 0];
        let mut current: i32 = 0;
        for line in lines {
            if let Ok(n) = line.parse::<i32>() {
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

    enum Move {
        Rock,
        Paper,
        Scissors,
    }

    impl Move {
        fn vs(&self, other: &Move) -> i32 {
            self.value()
                + match (self, other) {
                    (Move::Rock, Move::Scissors) => 6,
                    (Move::Paper, Move::Rock) => 6,
                    (Move::Scissors, Move::Paper) => 6,
                    (Move::Rock, Move::Rock) => 3,
                    (Move::Paper, Move::Paper) => 3,
                    (Move::Scissors, Move::Scissors) => 3,
                    (Move::Rock, Move::Paper) => 0,
                    (Move::Paper, Move::Scissors) => 0,
                    (Move::Scissors, Move::Rock) => 0,
                }
        }

        fn value(&self) -> i32 {
            match self {
                Move::Rock => 1,
                Move::Paper => 2,
                Move::Scissors => 3,
            }
        }
    }

    fn lose_draw_or_match(opponent: &Move, c: char) -> i32 {
        match (opponent, c) {
            // lose
            (Move::Rock, 'X') => Move::Scissors.vs(opponent),
            (Move::Paper, 'X') => Move::Rock.vs(opponent),
            (Move::Scissors, 'X') => Move::Paper.vs(opponent),
            // tie
            (Move::Rock, 'Y') => Move::Rock.vs(opponent),
            (Move::Paper, 'Y') => Move::Paper.vs(opponent),
            (Move::Scissors, 'Y') => Move::Scissors.vs(opponent),
            // win
            (Move::Rock, 'Z') => Move::Paper.vs(opponent),
            (Move::Paper, 'Z') => Move::Scissors.vs(opponent),
            (Move::Scissors, 'Z') => Move::Rock.vs(opponent),

            _ => unimplemented!(),
        }
    }

    pub fn run() -> String {
        let mut result: String = String::with_capacity(128);

        let file_string = std::fs::read_to_string("data/input_day02.txt").unwrap();
        let lines = file_string.lines();
        let (part_1, part_2): (i32, i32) = lines.clone().fold((0, 0), |(p1, p2), line| {
            let mut chars = line.chars();
            let opponent = match chars.next() {
                Some('A') => Move::Rock,
                Some('B') => Move::Paper,
                Some('C') => Move::Scissors,
                _ => unimplemented!(),
            };
            chars.next(); // skip the space
            let c = chars.next().unwrap();
            let me = match c {
                'X' => Move::Rock,
                'Y' => Move::Paper,
                'Z' => Move::Scissors,
                _ => unimplemented!(),
            };
            (p1 + me.vs(&opponent), p2 + lose_draw_or_match(&opponent, c))
        });

        writeln!(&mut result, "Day 02, Part 1: {}", part_1).unwrap();
        writeln!(&mut result, "Day 02, Part 2: {}", part_2).unwrap();
        result
    }
}
