use crate::maps::Position;
use crate::maps::Direction;
use Direction::*;

const VERBOSE:bool = true;

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

    // return None for "don't follow this path"
    fn get_cost_of_state(&self, start_state:State) -> Cost {
        let mut backlog:Vec<State> = Vec::new();
        let mut cache:HashMap<State,Cost> = HashMap::new();

        cache.insert(start_state, 0);
        backlog.push(start_state);

        // recursion termination at start point
        if self.map.at(start_state.0) == End {
            if VERBOSE { println!("Terminated at start");}
            return 0;
        }

        while backlog.len() > 0 {
            // extract element with minimum cost
            let min_cost = backlog.iter().map(|state| *cache.get(state).unwrap()).min().unwrap();
            let min_index = backlog.iter().position(|state| *cache.get(state).unwrap() == min_cost).unwrap();
            let state = backlog.swap_remove(min_index);
            let current_cost = min_cost;
            if VERBOSE { println!("Handle {:?} with cost = {}", state, current_cost);}

            for action in [Walk, TurnRight, TurnLeft] {
                if VERBOSE { println!("  try to do {:?}", action);}
                if let Some(after) = self.execute_action(state, action) {
                    let cost_this_way = cost_of_action(action) + current_cost;

                    // recursion termination
                    if self.map.at(after.0) == End {
                        if VERBOSE { println!("Terminated");}
                        if VERBOSE { self.print_cache(cache);}
                        return cost_this_way;
                    }

                    if let Some(&best_cost_up_to_now) = cache.get(&after) {
                        if cost_this_way < best_cost_up_to_now {
                            cache.insert(after, cost_this_way);
                            if VERBOSE { println!("  better cost for {:?}: {} < {}", after, cost_this_way, best_cost_up_to_now)}
                            backlog.push(after);
                        }
                    } else {
                        cache.insert(after, cost_this_way);
                        if VERBOSE { println!("  cost for {:?}: {}", after, cost_this_way)}
                        backlog.push(after);
                    }
                }
            }
        }

        panic!("Did not find any path to the end");
    }

    #[cfg(test)]
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

    assert_eq!(puzzle.get_cost_of_state(((13,1),Right)), 0);
    assert_eq!(puzzle.get_cost_of_state(((13,1),Up)), 0);
    let cost3 = puzzle.get_cost_of_state(((12,1),Right));
 //   puzzle.print_state();
    assert_eq!(cost3, 1);
    assert_eq!(puzzle.get_cost_of_state(((12,1),Up)), 1001);
    assert_eq!(puzzle.get_cost_of_state(((11,1),Up)), 1002);
    assert_eq!(puzzle.get_cost_of_state(((11,1),Left)), 2002);
    assert_eq!(puzzle.get_cost_of_state(((12,1),Right)), 1);
    assert_eq!(puzzle.get_cost_of_state(((11,3),Right)), 4008);

    assert_eq!(puzzle.get_cost_of_state(puzzle.get_start_state()), 7036);

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

    assert_eq!(puzzle.get_cost_of_state(((15,1),Right)), 0);
    assert_eq!(puzzle.get_cost_of_state(puzzle.get_start_state()), 11048);

}