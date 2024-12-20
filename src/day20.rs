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

type Action = (Direction,/* cheating: */ bool);

fn all_actions() -> [Action;8] {
    [
        (Up, false), (Right, false), (Down, false), (Left, false),
        (Up, true),  (Right, true),  (Down, true),  (Left, true)
    ]
}

type Cost = u32;

fn cost_of_action(action:Action) -> Cost {
    if action.1 { 2 } else { 1 }
}

type CheatsAllowed = u32;

type State = (Position, CheatsAllowed);

// a path is a set of actions from Start to End
type Path = Vec<Action>;

fn cost_of_path(path:&Path) -> Cost {
    path.iter().map(|&action| cost_of_action(action)).sum()
}

use std::collections::HashSet;
struct Puzzle {
    map:Map,
    path_without_cheating:Option<Path>,
    paths_with_cheating:Vec<Path>
}

impl Puzzle {
    fn read_input<'a>(map_lines:impl Iterator<Item=&'a str>) -> Puzzle {
        Puzzle {
            map:Map::from_strings(map_lines),
            path_without_cheating:None,
            paths_with_cheating:Vec::new()
        }
    }

    fn continue_path(&mut self, current_state:State, path_to_now:Path, been_there_to_now:&HashSet<Position>) {
        if VERBOSE { println!("At ({},{}), {} cheats left", current_state.0.0, current_state.0.1, current_state.1);}
        let mut been_there = been_there_to_now.clone();
        assert!(!been_there.contains(&current_state.0));
        been_there.insert(current_state.0);

        if self.map.at(current_state.0) == End {
            if current_state.1 > 0 {
                assert!(self.path_without_cheating.is_none());
                if VERBOSE { println!("  Finished without cheating, cost = {}", cost_of_path(&path_to_now));}
                self.path_without_cheating = Some(path_to_now);
            } else {
                if VERBOSE { println!("  Finished with cheating, cost = {}", cost_of_path(&path_to_now));}
                self.paths_with_cheating.push(path_to_now);
            }
            return;
        }

        for action in all_actions() {
            if VERBOSE { println!("  Try to do {:?}", action);}
            let next = self.execute_action(current_state, action);
            if next.is_none() { continue; }
            let next = next.unwrap();
            if been_there.contains(&next.0) {
                if VERBOSE { println!("  Been there. Done that.");}
                continue;
            }
            let mut new_path = path_to_now.clone();
            new_path.push(action);
            let level = new_path.len();
            if VERBOSE { println!("  Recurse at level {}", level);}
            self.continue_path(next, new_path, &been_there);
            if VERBOSE { println!("  Back from level {} on ({},{}), {} cheats left", level, current_state.0.0, current_state.0.1, current_state.1);}
        }

        if VERBOSE { println!("  Done with ({},{}), {} cheats left", current_state.0.0, current_state.0.1, current_state.1);}

    }

    fn create_all_paths(&mut self) {
        let start_state = self.get_start_state();
        let path_to_now = Vec::new();
        self.continue_path(start_state, path_to_now, &HashSet::new());
    }

    fn get_cheating_path_savings(&self) -> Vec<Cost> {
        let original_cost = cost_of_path(self.path_without_cheating.as_ref().unwrap());
        self.paths_with_cheating.iter().map(|path| original_cost - cost_of_path(path)).collect::<Vec<Cost>>()
    }

    fn get_start_state(&self) -> State {
        (self.map.find_first(Start).unwrap(), 1)
    }

    // none means action cannot be executed
    fn execute_action(&self, before:State, action:Action) -> Option<State> {
        if action.1 {
            if before.1 == 0 { return None; }
            let after1 = self.map.area.step(before.0, action.0);
            if after1.is_none() { return None; }
            let after1 = after1.unwrap();
            // if there is no wall we could have done this without cheating
            if self.map.at(after1) != Wall { return None; }
            let after2 = self.map.area.step(after1, action.0);
            if after2.is_none() { return None; }
            let after2 = after2.unwrap();
            if self.map.at(after2) == Wall { return None; }
            return Some((after2, before.1-1));
        } else {
            let after = self.map.area.step(before.0, action.0);
            if after.is_none() { return None; }
            let after = after.unwrap();
            if self.map.at(after) == Wall { return None; }
            return Some((after, before.1));
        }
    }


}

#[test]
fn test_puzzle1() {
    let input=
"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
    let mut puzzle = Puzzle::read_input(input.split('\n'));
    puzzle.create_all_paths();
    let start_pos = puzzle.get_start_state();
    assert_eq!(start_pos, ((1, 3), 1));
    assert_eq!(puzzle.execute_action(start_pos, (Up,false)), Some(((1,2), 1)));
    assert_eq!(puzzle.execute_action(start_pos, (Right,false)), None);
    assert_eq!(puzzle.execute_action(start_pos, (Right,true)), Some(((3,3), 0)));
    assert_eq!(puzzle.execute_action(start_pos, (Left,false)), None);
    assert_eq!(puzzle.execute_action(start_pos, (Left,true)), None);

    assert!(puzzle.path_without_cheating.is_some());
    assert_eq!(cost_of_path(puzzle.path_without_cheating.as_ref().unwrap()), 84);
    assert_eq!(puzzle.paths_with_cheating.len(), 14+14+2+4+2+3+5);

    let mut path_costs = puzzle.get_cheating_path_savings();
    path_costs.sort();
    assert_eq!(path_costs, vec![
        2,2,2,2,2,2,2,2,2,2,2,2,2,2,
        4,4,4,4,4,4,4,4,4,4,4,4,4,4,
        6,6,
        8,8,8,8,
        10,10,
        12,12,12,
        20, 36, 38, 40, 64
    ]);

}

