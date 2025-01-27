
type Level = i32;

//////////////////////////////////////////
/// Report
//////////////////////////////////////////

#[derive(Debug, PartialEq)]
struct Report {
    levels:Vec<Level>
}

type Reports = Vec<Report>;

impl Report {

    fn level(&self, index:usize, ignore:Option<usize>) -> Level {
        match ignore {
            Some(ig) => { if index >= ig { self.levels[index+1] } else { self.levels[index] } },
            None     => self.levels[index]
        }
    }

    fn is_slowly_changing(&self, min_diff:Level, max_diff:Level, tolerate_one: bool, ignore:Option<usize> ) -> bool {
        //println!("Check >= {} <= {} tolerate {} ", min_diff, max_diff, tolerate);

        let end = if ignore.is_some() { self.levels.len() - 2 } else { self.levels.len() - 1};
        for i in 0 .. end {

            //print!("{} ", next - last);
            let diff = self.level(i+1, ignore) - self.level(i, ignore);
            if diff > max_diff ||
               diff < min_diff {
                if tolerate_one {
                    return self.is_slowly_changing(min_diff, max_diff, false, Some(i))
                        || self.is_slowly_changing(min_diff, max_diff, false, Some(i + 1));
                    //print!("(tolerating {} - {} = {}) ", next, last, next - last);
                }
                //println!("fail");
                return false;
            }
        }
        //println!("ok");
        true
    }

    fn is_safe(&self, tolerate_one: bool) -> bool {
        self.is_slowly_changing( 1,  3, tolerate_one, None)
        ||
        self.is_slowly_changing(-3, -1, tolerate_one, None)

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
    assert_eq!(reports[0].is_safe(false), true);
    assert_eq!(reports[1].is_safe(false), false);
    assert_eq!(reports[2].is_safe(false), false);
    assert_eq!(reports[3].is_safe(false), false);
    assert_eq!(reports[4].is_safe(false), false);
    assert_eq!(reports[5].is_safe(false), true);

    assert_eq!(reports[0].is_safe(true), true);
    assert_eq!(reports[1].is_safe(true), false);
    assert_eq!(reports[2].is_safe(true), false);
    assert_eq!(reports[3].is_safe(true), true);
    assert_eq!(reports[4].is_safe(true), true);
    assert_eq!(reports[5].is_safe(true), true);

    assert_eq!(parse_report("100 1 2 3").is_safe(true), true);
    assert_eq!(parse_report("100 1 2 3 4").is_safe(true), true);
    assert_eq!(parse_report("1 2 3 99").is_safe(true), true);
    assert_eq!(parse_report("40 41 43 44 47 47").is_safe(true), true);
    assert_eq!(parse_report("1 3 2 3 4").is_safe(true), true);
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

    let safe_report_count1:u32 = reports.iter().map( |report| match report.is_safe(false) { true => 1, false => 0 }).sum();
    println!("Day 2, Part 1: Number of safe reports is {} of {}", safe_report_count1, reports.len());

    let safe_report_count2:u32 = reports.iter().map( |report| match report.is_safe(true) { true => 1, false => 0 }).sum();
    println!("Day 2, Part 2: Number of safe reports is {} if one bad level is tolerated", safe_report_count2);

}