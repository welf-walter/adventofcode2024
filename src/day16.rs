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
use std::collections::HashSet;
struct Puzzle {
    map:Map,
    // already evaluated
    cache:HashMap<State,Cost>,
    // currently in evaluation
    backlog:HashSet<State>
}

impl Puzzle {
    fn read_input<'a>(map_lines:impl Iterator<Item=&'a str>) -> Puzzle {
        Puzzle {
            map:Map::from_strings(map_lines),
            cache:HashMap::new(),
            backlog:HashSet::new()
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

    fn get_cost_of_state(&mut self, state:State) -> Cost {
        // ... todo
        if let Some(cached) = self.cache.get(&state) {
            if VERBOSE { println!("Cached: {:?} -> {}", state, *cached)}
            return *cached;
        }

        // recursion termination
        if self.map.at(state.0) == End {
            if VERBOSE { println!("Terminated at {:?}", state);}
            return 0;
        }

        assert!(!self.backlog.contains(&state));
        self.backlog.insert(state);
        // first try to walk, because it is cheaper
        let mut options:Vec<(Action, Cost)> = Vec::new();
        for action in [Walk, TurnRight, TurnLeft] {
            if VERBOSE { println!("At {:?} do {:?}", state, action);}
            if let Some(after) = self.execute_action(state, action) {
                // don't evaluate if already on stack
                if self.backlog.contains(&after) {
                    if VERBOSE { println!("At {:?} do {:?} -> skip", state, action);}
                    continue;
                }
                let cost = cost_of_action(action) + self.get_cost_of_state(after);
                if VERBOSE { println!("At {:?} do {:?} -> {}", state, action, cost)}
                options.push((action, cost));
            }
        }
        self.backlog.remove(&state);
        let best_option = options.iter().min_by_key(|(_action,cost)| cost).unwrap();
        let best_cost = best_option.1;
        self.cache.insert(state, best_cost);
        best_cost
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
    let mut puzzle = Puzzle::read_input(input.split('\n'));
    let start_pos = puzzle.get_start_state();
    assert_eq!(start_pos, ((1, 13),Right));
    assert_eq!(puzzle.execute_action(start_pos, Walk), Some(((2,13), Right)));
    let r = puzzle.execute_action(start_pos, TurnRight).unwrap();
    assert_eq!(r, ((1,13), Down));
    assert_eq!(puzzle.execute_action(r, Walk), None);

    assert_eq!(puzzle.get_cost_of_state(((13,1),Right)), 0);
    assert_eq!(puzzle.get_cost_of_state(((12,1),Right)), 1);
    assert_eq!(puzzle.get_cost_of_state(((11,1),Left)), 1002);
    assert_eq!(puzzle.get_cost_of_state(((12,1),Right)), 1);
    assert_eq!(puzzle.get_cost_of_state(((11,3),Right)), 4008);

}