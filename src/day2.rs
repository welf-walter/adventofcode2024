
type Level = u32;

//////////////////////////////////////////
/// Report
//////////////////////////////////////////

#[derive(Debug, PartialEq)]
struct Report {
    levels:Vec<Level>
}

type Reports = Vec<Report>;

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

