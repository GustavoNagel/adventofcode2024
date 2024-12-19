use std::env;
use std::fs;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;


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
        9 => day09::run(contents, part),
        10 => day10::run(contents, part),
        11 => day11::run(contents, part),
        12 => day12::run(contents, part),
        13 => day13::run(contents, part),
        _ => println!("Failed because code for this day wasn't found!"),
    }
}
