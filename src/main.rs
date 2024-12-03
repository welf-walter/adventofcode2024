mod day1;
mod day2;
mod day3;

const NUMBER_OF_DAYS : u32 = 3;

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
            otherday=>println!("Unknown day {}", otherday)
        }
    }
}
