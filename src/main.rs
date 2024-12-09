mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

const NUMBER_OF_DAYS : u32 = 8;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// The days to handle
    days: Vec<u32>
}

fn main() {
    let args = Cli::parse();

    let days = if !args.days.is_empty() { args.days } else { (1..NUMBER_OF_DAYS+1).collect() };

    for day in days {
        match day {
            1=>day1::puzzle(),
            2=>day2::puzzle(),
            3=>day3::puzzle(),
            4=>day4::puzzle(),
            5=>day5::puzzle(),
            6=>day6::puzzle(),
            7=>day7::puzzle(),
            8=>day8::puzzle(),
            otherday=>println!("Unknown day {}", otherday)
        }
    }
}
