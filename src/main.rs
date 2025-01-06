mod maps;
mod optimize;
mod helper;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
// mod day13;
// mod day14;
mod day15;
mod day16;
// ...
mod day20;
mod day23;

const NUMBER_OF_DAYS : u32 = 20;

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
            9=>day9::puzzle(),
            10=>day10::puzzle(),
            11=>{}, //day11::puzzle(),
            12=>day12::puzzle(),
            13=>{}, //day13::puzzle(),
            14=>{}, //day14::puzzle(),
            15=>day15::puzzle(),
            16=>day16::puzzle(),
            17=>{}, //day14::puzzle(),
            18=>{}, //day14::puzzle(),
            19=>{}, //day14::puzzle(),
            20=>day20::puzzle(),
            21=>{}, //day21::puzzle(),
            22=>{}, //day22::puzzle(),
            23=>day23::puzzle(),

            otherday=>println!("Unknown day {}", otherday)
        }
    }
}
