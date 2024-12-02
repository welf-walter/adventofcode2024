use std::slice::Iter;

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

    fn is_slowly_changing(mut iter:Iter<Level>, min_diff:Level, max_diff:Level, tolerate: u32) -> bool {
        //println!("Check >= {} <= {} tolerate {} ", min_diff, max_diff, tolerate);
        let mut tol = tolerate;
        let mut last = iter.next().unwrap();
        for next in iter {
            //print!("{} ", next - last);
            if next - last > max_diff ||
               next - last < min_diff {
                if tol > 0 {
                    //print!("(tolerating {} - {} = {}) ", next, last, next - last);
                    tol -= 1;
                    continue;
                }
                //println!("fail");
                return false;
            }
            last = next;
        }
        //println!("ok");
        true
    }

    fn is_iter_safe(mut iter:Iter<Level>, tolerate: u32) -> bool {
        if Self::is_slowly_changing(iter.clone(),  1,  3, tolerate) { return true };
        if Self::is_slowly_changing(iter.clone(), -3, -1, tolerate) { return true };
        if tolerate > 0 { 
            println!("Not safe: {:?} (tolerate = {})", iter, tolerate);
            iter.next();
            return Self::is_iter_safe(iter, tolerate - 1); 
        };
        println!("Not safe: {:?} (tolerate = {})", iter, tolerate);
        false
    }

    fn is_safe(&self, tolerate: u32) -> bool {
        Self::is_iter_safe(self.levels.iter(), tolerate)
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
    assert_eq!(reports[0].is_safe(0), true);
    assert_eq!(reports[1].is_safe(0), false);
    assert_eq!(reports[2].is_safe(0), false);
    assert_eq!(reports[3].is_safe(0), false);
    assert_eq!(reports[4].is_safe(0), false);
    assert_eq!(reports[5].is_safe(0), true);

    assert_eq!(reports[0].is_safe(1), true);
    assert_eq!(reports[1].is_safe(1), false);
    assert_eq!(reports[2].is_safe(1), false);
    assert_eq!(reports[3].is_safe(1), true);
    assert_eq!(reports[4].is_safe(1), true);
    assert_eq!(reports[5].is_safe(1), true);

    assert_eq!(parse_report("100 1 2 3").is_safe(1), true);
    assert_eq!(parse_report("1 22 2 99 3").is_safe(2), true);
    assert_eq!(parse_report("100 1 2 3 999").is_safe(2), true);
    assert_eq!(parse_report("100 999 1 2 3").is_safe(2), true);
    assert_eq!(parse_report("40 41 43 44 47 47").is_safe(1), true);
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

    let safe_report_count1:u32 = reports.iter().map( |report| match report.is_safe(0) { true => 1, false => 0 }).sum();
    println!("Day 2, Part 1: Number of safe reports is {} of {}", safe_report_count1, reports.len());

    let safe_report_count2:u32 = reports.iter().map( |report| match report.is_safe(1) { true => 1, false => 0 }).sum();
    println!("Day 2, Part 2: Number of safe reports is {} if one bad level is tolerated", safe_report_count2);

}