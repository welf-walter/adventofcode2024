type Position = (u32,u32);
type Velocity = (i32,i32);
type Width = u32;
type Height = u32;

struct Bathroom {
    width:Width,
    height:Height
}

type QuadrantCounter = (u32,u32,u32,u32);

impl Bathroom {
    fn get_quadrant_counter(&self, position:Position) -> QuadrantCounter {
        if position.0 > self.width / 2 && position.1 < self.height / 2 { (1,0,0,0) } else
        if position.0 < self.width / 2 && position.1 < self.height / 2 { (0,1,0,0) } else
        if position.0 > self.width / 2 && position.1 > self.height / 2 { (0,0,1,0) } else
        if position.0 < self.width / 2 && position.1 > self.height / 2 { (0,0,0,1) } else {
            (0,0,0,0)
        }
    }
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

    // move the robot {counter} times through {bathroom}
    fn move_robot(&self, bathroom:&Bathroom, counter:u32) -> Position {
        let vx = self.velocity.0;
        let vy = self.velocity.1;
        let dx = if vx > 0 { vx as u32 } else { (bathroom.width  as i32 + vx) as u32 };
        let dy = if vy > 0 { vy as u32 } else { (bathroom.height as i32 + vy) as u32 };
        ((self.position.0 + counter * dx ) % bathroom.width,
         (self.position.1 + counter * dy ) % bathroom.height)
    }

}

fn get_safety_factor<Iter:Iterator<Item=Position>>(bathroom:&Bathroom, positions:Iter) -> u32 {
    let mut counters:QuadrantCounter = (0,0,0,0);
    for pos in positions {
        let counter = bathroom.get_quadrant_counter(pos);
        counters.0 += counter.0;
        counters.1 += counter.1;
        counters.2 += counter.2;
        counters.3 += counter.3;
    }
    counters.0 * counters.1 * counters.2 * counters.3
}

#[test]
fn test_move() {
    let bathroom = Bathroom{width:11, height:7};
    let robot = Robot::from_string("p=2,4 v=2,-3");
    assert_eq!(robot.move_robot(&bathroom, 0), (2,4) );
    assert_eq!(robot.move_robot(&bathroom, 1), (4,1) );
    assert_eq!(robot.move_robot(&bathroom, 2), (6,5) );
    assert_eq!(robot.move_robot(&bathroom, 3), (8,2) );
    assert_eq!(robot.move_robot(&bathroom, 4), (10,6) );
    assert_eq!(robot.move_robot(&bathroom, 5), (1,3) );
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

    let bathroom = Bathroom{width:11, height:7};
    let positions = robots.iter().map(|robot| robot.move_robot(&bathroom, 100));
    assert_eq!(get_safety_factor(&bathroom, positions), 12);
}

//////////////////////////////////////////
/// Puzzle
//////////////////////////////////////////

pub fn puzzle() {
    let lines = crate::helper::read_file("input/day14.txt");
    let robots = lines.iter().map(|line| Robot::from_string(line)).collect::<Vec<Robot>>();

    let bathroom = Bathroom{width:101, height:103};
    let positions = robots.iter().map(|robot| robot.move_robot(&bathroom, 100));
    let safety_factor = get_safety_factor(&bathroom, positions);
    println!("Day 14, Part 1: Safety factor after moving {} robots for 100 seconds is {}", robots.len(), safety_factor);

}
