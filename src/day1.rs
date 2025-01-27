
type LocationID = u32;

//////////////////////////////////////////
/// Input parsing
//////////////////////////////////////////

fn read_input<'a>(lines:impl Iterator<Item=&'a str>) -> (Vec<LocationID>, Vec<LocationID>) {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in lines {
        let mut split = line.split("   ");
        left.push(split.next().unwrap().parse::<LocationID>().unwrap());
        right.push(split.next().unwrap().parse::<LocationID>().unwrap());
        assert!(split.next().is_none());
    }
    (left, right)
}

#[test]
fn test_read_input()
{
    let input1 =
"3   4
4   3
2   5
1   3
3   9
3   3";
    let (left, right) = read_input(input1.split("\n"));
    assert_eq!(left, vec![3,4,2,1,3,3]);
    assert_eq!(right, vec![4,3,5,3,9,3]);
}

//////////////////////////////////////////
/// Distance
//////////////////////////////////////////

use std::cmp;

fn calculate_total_distance(left_in:&Vec<LocationID>, right_in:&Vec<LocationID>) -> LocationID {
    let mut left_sorted = left_in.clone();
    left_sorted.sort();
    let mut right_sorted = right_in.clone();
    right_sorted.sort();
    let mut total_distance = 0;
    let mut liter = left_sorted.iter();
    let mut riter = right_sorted.iter();
    loop {
        let l = liter.next();
        let r = riter.next();
        //println!("({:?},{:?})", l,r);
        match (l,r) {
            (Some(left_value), Some(right_value)) => total_distance += cmp::max(left_value, right_value) - cmp::min(left_value, right_value),
            (None, None) => return total_distance,
            (_, _) => panic!("Unexpected ({:?},{:?})", l,r)
        }
    }
}

#[test]
fn test_distance() {
    let input1 =
"3   4
4   3
2   5
1   3
3   9
3   3";
    let (left, right) = read_input(input1.split("\n"));
    assert_eq!(calculate_total_distance(&left, &right), 11);
}

//////////////////////////////////////////
/// Similarity
//////////////////////////////////////////

fn calculate_similarity(left:&Vec<LocationID>, right:&Vec<LocationID>) -> LocationID {
    let mut similarity = 0;
    for l in left.iter() {
        for r in right.iter() {
            if *l == *r {
                similarity += *l;
            }
        }
    }
    similarity
}

#[test]
fn test_similarity() {
    let input1 =
"3   4
4   3
2   5
1   3
3   9
3   3";
    let (left, right) = read_input(input1.split("\n"));
    assert_eq!(calculate_similarity(&left, &right), 31);
}

//////////////////////////////////////////
/// Puzzle
//////////////////////////////////////////

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn puzzle() {
    let file = File::open("input/day1.txt").expect("Could not open input/day1.txt");
    let reader = BufReader::new(file);

    let lines:Vec<String> = reader.lines().map( |line| line.unwrap() ).collect();
    let (left, right) = read_input(lines.iter().map( |line| line.as_str() ));

    let total_distance = calculate_total_distance(&left, &right);
    println!("Day 1, Part 1: Sum of distance of sorted pairs is {}", total_distance);

    let total_similarity = calculate_similarity(&left, &right);
    println!("Day 1, Part 2: Sum of equal values is {}", total_similarity);

}