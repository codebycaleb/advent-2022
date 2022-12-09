// Shamelessly stolen from https://www.forrestthewoods.com/blog/learning-rust-via-advent-of-code/

use rayon::prelude::*;
use std::io;
// use std::io::{Read, Write};
use std::io::Write;
use std::time::{Duration, Instant};

type DayFn = fn() -> String;

#[allow(dead_code)]
fn run_one(day: &dyn Fn() -> String, num_loops: usize) -> (Vec<String>, Duration) {
    let mut results = Vec::with_capacity(num_loops);

    let start = Instant::now();
    for _ in 0..num_loops {
        let result = day();
        results.push(result);
    }

    (results, start.elapsed() / num_loops as u32)
}

#[allow(dead_code)]
fn run_all(days: std::slice::Iter<'_, DayFn>, num_loops: usize) -> (Vec<String>, Duration) {
    let mut results: Vec<String> = Vec::with_capacity(num_loops * 25);

    let start = Instant::now();
    for _ in 0..num_loops {
        for f in days.clone() {
            results.push(f());
        }
    }
    let elapsed = start.elapsed();

    results.push(format!("\nTotal time (single-threaded): {:?}\n", elapsed));
    (results, elapsed / num_loops as u32)
}

#[allow(dead_code)]
fn run_all_parallel(
    days: rayon::slice::Iter<'_, DayFn>,
    num_loops: usize,
) -> (Vec<String>, Duration) {
    let start = Instant::now();

    let mut results: Vec<String> = (0..num_loops)
        .into_par_iter()
        .flat_map(|_| days.clone().map(|f| f()).collect::<Vec<String>>())
        .collect();
    let elapsed = start.elapsed();

    results.push(format!("\nTotal time (parallel): {:?}\n", elapsed));
    (results, elapsed / num_loops as u32)
}

#[allow(dead_code)]
fn bench(days: std::slice::Iter<'_, DayFn>, num_loops: usize) -> (Vec<String>, Duration) {
    let mut results: Vec<String> = Vec::new();
    let mut time = Duration::new(0, 0);

    for (i, day) in days.clone().enumerate() {
        let day_result = run_one(day, num_loops);
        results.push(format!("Day {:?} - {:?}\n", i + 1, day_result.1));
        time += day_result.1;
    }

    (results, time)
}

fn main() {
    #[allow(unused_variables)]
    let days = [
        advent::day01::run,
        advent::day02::run,
        advent::day03::run,
        advent::day04::run,
    ];

    let num_loops = 1;

    let result = run_all(days.iter(), num_loops);
    //let result = run_all_parallel(days.par_iter(), num_loops);
    //let result = run_one(&advent::day15::run, num_loops);
    //let result = bench(days.iter(), num_loops);

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for result in result.0 {
        write!(handle, "{}", result).unwrap();
    }
    writeln!(handle, "Average Time: {:?}\n", result.1).unwrap();

    // ------------------------------------------------------------
    // Do not modify below this line
    // ------------------------------------------------------------
    // println!("Hit enter to continue");

    // let mut input = [0; 1];
    // let _ = io::stdin().read_exact(&mut input);
}
