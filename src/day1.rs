use std::vec;

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
        match (l,r) {
            (Some(left_value), Some(right_value)) => total_distance += right_value - left_value,
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
/// Puzzle
//////////////////////////////////////////

pub fn puzzle() {
    ;;;
}