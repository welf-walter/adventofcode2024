use std::collections::HashSet;

use crate::maps::Position;
use crate::maps::Direction;
use crate::optimize::get_all_best_paths;
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

use crate::optimize::Cost;
use crate::optimize::ActionTrait;
use crate::optimize::Problem;
use crate::optimize::get_cost_of_state;

impl ActionTrait for Action {
    fn all_actions() -> &'static [Self] {
        &[Walk, TurnRight, TurnLeft]
     }
}

type State = (Position,Direction);

struct Puzzle {
    map:Map
}

impl Puzzle {
    fn read_input<'a>(map_lines:impl Iterator<Item=&'a str>) -> Puzzle {
        Puzzle {
            map:Map::from_strings(map_lines)
        }
    }

    fn get_start_state(&self) -> State {
        (self.map.find_first(Start).unwrap(), Right)
    }
}

impl Problem for Puzzle {

    type Action = Action;
    type State = State;

    fn is_end_state(&self, state:&State) -> bool {
        self.map.at(state.0) == End
    }

    fn execute_action(&self, before:State, action:Action) -> Option<State> {
        match action {
            TurnLeft => {
                Some((before.0, before.1.turn_left()))
            },
            TurnRight => {
                Some((before.0, before.1.turn_right()))
            },
            Walk => {
                match self.map.area.step(before.0, before.1)
                {
                    Some(nextpos) => {
                        if self.map.at(nextpos) == Wall {
                            None
                        } else {
                            Some((nextpos, before.1))
                        }
                    },
                    None => None
                }
            }
        }
    }

    fn cost(&self, action:Action) -> Cost {
        match action {
            Walk      => 1,
            TurnLeft  => 1000,
            TurnRight => 1000
        }
    }

    /*
    fn print_cache(&self, cache:HashMap<State,Cost>) {
        for y in 0..self.map.area.height {
            for x in 0..self.map.area.width {
                let min_cost =
                    [Right, Down, Left, Up].iter()
                    .map(|&direction| cache.get(&((x,y),direction)))
                    .filter(|option| option.is_some())
                    .map(|option| option.unwrap())
                    .min();
                if let Some(cost) = min_cost {
                    print!("{:5} ", cost);
                } else {
                    print!(" ???  ");
                }
            }
            println!("");
        }
    }
    */

}

fn count_tiles_which_are_part_of_any_best_path(puzzle:&Puzzle) -> usize {
    let best_paths = get_all_best_paths(puzzle, puzzle.get_start_state());
    let mut relevant_positions : HashSet<Position> = HashSet::new();
    for path in best_paths {
        let mut state = puzzle.get_start_state();
        relevant_positions.insert(state.0);
        for action in path {
            state = puzzle.execute_action(state, action).unwrap();
            relevant_positions.insert(state.0);
        }
    }

    relevant_positions.len()

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
    assert_eq!(puzzle.execute_action(start_pos, Walk), Some(((2,13), Right)));
    let r = puzzle.execute_action(start_pos, TurnRight).unwrap();
    assert_eq!(r, ((1,13), Down));
    assert_eq!(puzzle.execute_action(r, Walk), None);

    assert_eq!(get_cost_of_state(&puzzle, ((13,1),Right)), 0);
    assert_eq!(get_cost_of_state(&puzzle, ((13,1),Up)), 0);
    let cost3 = get_cost_of_state(&puzzle, ((12,1),Right));
 //   puzzle.print_state();
    assert_eq!(cost3, 1);
    assert_eq!(get_cost_of_state(&puzzle, ((12,1),Up)), 1001);
    assert_eq!(get_cost_of_state(&puzzle, ((11,1),Up)), 1002);
    assert_eq!(get_cost_of_state(&puzzle, ((11,1),Left)), 2002);
    assert_eq!(get_cost_of_state(&puzzle, ((12,1),Right)), 1);
    assert_eq!(get_cost_of_state(&puzzle, ((11,3),Right)), 4008);

    assert_eq!(get_cost_of_state(&puzzle, puzzle.get_start_state()), 7036);

    assert_eq!(count_tiles_which_are_part_of_any_best_path(&puzzle), 45);

}

#[test]
fn test_puzzle2() {
    let input=
"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
    let puzzle = Puzzle::read_input(input.split('\n'));

    assert_eq!(get_cost_of_state(&puzzle, ((15,1),Right)), 0);
    assert_eq!(get_cost_of_state(&puzzle, puzzle.get_start_state()), 11048);

    assert_eq!(count_tiles_which_are_part_of_any_best_path(&puzzle), 64);

}

//////////////////////////////////////////
/// Puzzle
//////////////////////////////////////////

pub fn puzzle() {
    let lines = crate::helper::read_file("input/day16.txt");

    let puzzle = Puzzle::read_input(lines.iter().map(|line| line.as_str()));
    let costs = get_cost_of_state(&puzzle, puzzle.get_start_state());

    println!("Day 16, Part 1: Lowest score to move from Start to End is {}", costs);
    println!("Day 16, Part 2: Number of tiles part of any best path is {}", count_tiles_which_are_part_of_any_best_path(&puzzle));
}
