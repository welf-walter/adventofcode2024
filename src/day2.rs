
type Level = u32;

//////////////////////////////////////////
/// Report
//////////////////////////////////////////

#[derive(Debug, PartialEq)]
struct Report {
    levels:Vec<Level>
}

type Reports = Vec<Report>;

impl Report {

    fn is_slowly_increasing(&self) -> bool {
        let mut iter = self.levels.iter();
        let mut last = iter.next().unwrap();
        for next in iter {
            if next <= last { return false; }
            if next - last > 3 { return false; }
            last = next;
        }
        true
    }

    fn is_slowly_decreasing(&self) -> bool {
        let mut iter = self.levels.iter();
        let mut last = iter.next().unwrap();
        for next in iter {
            if next >= last { return false; }
            if last - next > 3 { return false; }
            last = next;
        }
        true
    }

    fn is_safe(&self) -> bool {
        self.is_slowly_increasing()
        ||
        self.is_slowly_decreasing()
    }
}

#[test]
fn test_is_safe() {
    let input1 =
"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    let reports = parse_reports(input1.split("\n"));
    assert_eq!(reports[0].is_safe(), true);
    assert_eq!(reports[1].is_safe(), false);
    assert_eq!(reports[2].is_safe(), false);
    assert_eq!(reports[3].is_safe(), false);
    assert_eq!(reports[4].is_safe(), false);
    assert_eq!(reports[5].is_safe(), true);
}

//////////////////////////////////////////
/// Parsing
//////////////////////////////////////////

fn parse_report(line:&str) -> Report {
    let mut levels:Vec<Level> = Vec::new();
    for level_str in line.split(" ") {
        levels.push(level_str.parse::<Level>().unwrap());
    }
    Report { levels:levels }
}

fn parse_reports<'a>(lines:impl Iterator<Item=&'a str>) -> Reports {
    let mut reports:Vec<Report> = Vec::new();
    for line in lines {
        reports.push(parse_report(line));
    }
    reports
}

#[test]
fn test_parse() {
    let input1 =
"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    let reports = parse_reports(input1.split("\n"));
    assert_eq!(reports,
        vec![
            Report { levels:vec![7,6,4,2,1] },
            Report { levels:vec![1,2,7,8,9] },
            Report { levels:vec![9,7,6,2,1] },
            Report { levels:vec![1,3,2,4,5] },
            Report { levels:vec![8,6,4,4,1] },
            Report { levels:vec![1,3,6,7,9] }
        ]);
}

//////////////////////////////////////////
/// Puzzle
//////////////////////////////////////////

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn puzzle() {
    let file = File::open("input/day2.txt").expect("Could not open input/day2.txt");
    let reader = BufReader::new(file);

    let lines:Vec<String> = reader.lines().map( |line| line.unwrap() ).collect();
    let reports = parse_reports(lines.iter().map( |line| line.as_str() ));
    
    let safe_report_count:u32 = reports.iter().map( |report| match report.is_safe() { true => 1, false => 0 }).sum();
    println!("Day 2, Part 1: Number of safe reports is {}", safe_report_count);

}