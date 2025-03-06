use std::env;
use std::fs;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day: &i8 = &args[1].parse().unwrap();
    let part: &i8 = &args[2].parse().unwrap();
    let path: String = format!("./input_files/day{day:02}.txt");

    if day < &1 || day > &25 { panic!("Day argument must be between 1 and 25") }
    if part != &1 && part != &2 { panic!("Part argument must be 1 or 2") }
    println!("Running script for day {day}, part {part}");

    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file. Check if the given file name actually exist");

    match day {
        1 => day01::run(contents, part),
        2 => day02::run(contents, part),
        3 => day03::run(contents, part),
        4 => day04::run(contents, part),
        5 => day05::run(contents, part),
        6 => day06::run(contents, part),
        7 => day07::run(contents, part),
        8 => day08::run(contents, part),
        9 => day09::run(contents, part),
        10 => day10::run(contents, part),
        11 => day11::run(contents, part),
        12 => day12::run(contents, part),
        13 => day13::run(contents, part),
        14 => day14::run(contents, part),
        15 => day15::run(contents, part),
        16 => day16::run(contents, part),
        17 => day17::run(contents, part),
        18 => day18::run(contents, part),
        19 => day19::run(contents, part),
        20 => day20::run(contents, part),
        21 => day21::run(contents, part),
        22 => day22::run(contents, part),
        23 => day23::run(contents, part),
        24 => day24::run(contents, part),
        _ => println!("Failed because code for this day wasn't found!"),
    }
}
