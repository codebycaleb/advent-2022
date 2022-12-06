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

    pub fn run() -> String {
        let mut result: String = String::with_capacity(128);
        writeln!(&mut result, "Day 02, Part 1: {}", "part_1").unwrap();
        writeln!(&mut result, "Day 02, Part 2: {}", "part_2").unwrap();
        result
    }
}
