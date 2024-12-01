use std::vec;

type LocationID = u32;

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
/// Puzzle
//////////////////////////////////////////

pub fn puzzle() {
    ;;;
}