type Position = (u32,u32);
type Velocity = (i32,i32);
type Width = u32;
type Height = u32;

struct Bathroom {
    width:Width,
    height:Height
}

#[derive(Debug, PartialEq)]
struct Robot {
    position:Position,
    velocity:Velocity
}

use regex::Regex;

impl Robot {

    // "p=0,4 v=3,-3"
    fn from_string(line:&str) -> Robot {

        let r = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
        let caps = r.captures(line).unwrap();
        let px = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let py = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
        let vx = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
        let vy = caps.get(4).unwrap().as_str().parse::<i32>().unwrap();

        Robot { position:(px,py),velocity:(vx,vy) }

    }
}

#[test]
fn test_example() {
    let input1 =
"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
    let robots = input1.split('\n').map(|line| Robot::from_string(line)).collect::<Vec<Robot>>();
    assert_eq!(robots.len(), 12);
    assert_eq!(robots[0], Robot{position:(0,4),velocity:(3,-3)});
    assert_eq!(robots[1], Robot{position:(6,3),velocity:(-1,-3)});
}