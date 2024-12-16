use crate::maps::Position;
use crate::maps::Direction;
use Direction::*;

#[derive(Debug, PartialEq, Clone, Copy)]
enum MapElement {
    Space,
    Wall,
    Start,
    End
}

use MapElement::*;

impl crate::maps::FromChar for MapElement {
    fn from_char(c:char) -> Self {
        match c {
            '.' => Space,
            '#' => Wall,
            'S' => Start,
            'E' => End,
            _ => panic!("Unexpected character {} for TestEnum", c)
        }
    }
}

use crate::maps::PixelMap;
type Map = PixelMap<MapElement>;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Action {
    Walk,
    TurnLeft,
    TurnRight
}
use Action::*;

type Cost = u32;

fn cost_of_action(action:Action) -> Cost {
    match action {
        Walk      => 1,
        TurnLeft  => 1000,
        TurnRight => 1000
    }
}

type State = (Position,Direction);

use std::collections::HashMap;
struct Puzzle {
    map:Map,
    cache:HashMap<State,Cost>
}

impl Puzzle {
    fn read_input<'a>(map_lines:impl Iterator<Item=&'a str>) -> Puzzle {
        Puzzle {
            map:Map::from_strings(map_lines),
            cache:HashMap::new()
        }
    }

    fn get_start_state(&self) -> State {
        (self.map.find_first(Start).unwrap(), Right)
    }

    fn get_cost_of_state(&mut self, state:State) -> Cost {
        // ... todo
        42
    }
}

#[test]
fn test_puzzle1() {
    let input=
"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
    let puzzle = Puzzle::read_input(input.split('\n'));
    let start_pos = puzzle.get_start_state();
    assert_eq!(start_pos, ((1, 13),Right));
}