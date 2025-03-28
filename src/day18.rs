use crate::maps::Direction;
use crate::optimize::get_cost_of_state;
use crate::maps::FromChar;
use crate::maps::Position;
use crate::maps::PixelMap;
use crate::optimize::Problem;
type Positions = Vec<Position>;

const VERBOSE:bool = false;

fn parse_input(lines:Vec<&str>) -> Positions {
    lines.iter().map(|&line|
        {
            let mut split = line.split(',');
            (split.next().unwrap().parse().unwrap(),
             split.next().unwrap().parse().unwrap())
        }
    ).collect()
}

impl FromChar for bool {
    fn from_char(c:char) -> Self {
        match c {
            '.' => false,
            '#' => true,
            _ => unreachable!()
        }
    }
}

fn drop_n(map:&mut PixelMap<bool>, positions:&Positions, n:usize) {
    let mut iter = positions.iter();
    for _ in 0..n {
        let pos = iter.next().unwrap();
        map.set_at(*pos, true);
    }
}

struct Maze {
    map:PixelMap<bool>
}

impl Maze {
    const START_STATE:Position = (0,0);
}

impl Problem for Maze {
    type State = Position;
    type Action = Direction;
    fn is_end_state(&self, state:&Self::State) -> bool {
        state.0 == self.map.width() - 1 &&
        state.1 == self.map.height() - 1
    }
    fn execute_action(&self, before:Self::State, action:Self::Action) -> Option<Self::State> {
        if let Some(newpos) = self.map.area.step(before, action) {
            if self.map.at(newpos) {
                None
            } else {
                Some(newpos)
            }
        } else {
            None
        }
    }

    fn cost(&self, _action:Self::Action) -> crate::optimize::Cost {
        1
    }

    fn all_actions(&self) -> Vec<Direction> {
        vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right]
    }

}

fn get_blocking_position(initialmap:PixelMap<bool>, positions:&Positions) -> Position {
    let mut problem = Maze{map:initialmap};
    for &pos in positions {
        problem.map.set_at(pos, true);
        let cost = get_cost_of_state(&problem, Maze::START_STATE);
        if VERBOSE { println!("Cost = {}", cost);}
        if cost == u32::MAX {
            return pos
        }
    }
    unreachable!();
}

#[cfg(test)]
fn input1() -> &'static str {
"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"
}


#[test]
fn test_example1() {
    let input = input1();
    let lines = input.split('\n').collect();
    let positions = parse_input(lines);
    assert_eq!(positions[3], (3,0));

    let initialmap = PixelMap::<bool>::new(7,7,false);

    let problem0 = Maze{map:initialmap.clone()};
    let cost0 = get_cost_of_state(&problem0, Maze::START_STATE);
    assert_eq!(cost0, 6+6);

    let mut map = initialmap.clone();
    drop_n(&mut map, &positions, 12);
    assert_eq!(map.at((3,0)), true);
    assert_eq!(map.at((1,2)), false);

    let problem1 = Maze{map};
    let cost1 = get_cost_of_state(&problem1, Maze::START_STATE);
    assert_eq!(cost1, 22);

    assert_eq!(get_blocking_position(initialmap, &positions), (6,1));
}

//////////////////////////////////////////
/// Puzzle
//////////////////////////////////////////

pub fn puzzle() {
    let lines = crate::helper::read_file("input/day18.txt");
    let positions = parse_input(lines.iter().map(|line| line.as_str()).collect());
    let initialmap = PixelMap::<bool>::new(71,71,false);

    let mut map1 = initialmap.clone();
    drop_n(&mut map1, &positions, 1024);
    let problem = Maze{map:map1};
    let cost = get_cost_of_state(&problem, Maze::START_STATE);

    println!("Day 16, Part 1: Minimum number of steps to reach output after 1024 bytes is {}", cost);

    let blocking_pos = get_blocking_position(initialmap, &positions);
    println!("Day 16, Part 1: Blocking position is {},{}", blocking_pos.0, blocking_pos.1);
}
