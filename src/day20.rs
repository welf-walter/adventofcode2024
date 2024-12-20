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

use std::collections::HashMap;
use std::collections::HashSet;
struct Puzzle {
    map:Map,
    path_without_cheating:Option<Path>,
    paths_with_cheating:Vec<Path>,
    states_handled:HashSet<State>
}

impl Puzzle {
    fn read_input<'a>(map_lines:impl Iterator<Item=&'a str>) -> Puzzle {
        Puzzle {
            map:Map::from_strings(map_lines),
            path_without_cheating:None,
            paths_with_cheating:Vec::new(),
            states_handled:HashSet::new()
        }
    }

    fn continue_path(&mut self, current_state:State, path_to_now:Path) {
        self.states_handled.insert(current_state);

        if self.map.at(current_state.0) == End {
            if current_state.1 > 0 {
                assert!(self.path_without_cheating.is_none());
                self.path_without_cheating = Some(path_to_now);
            } else {
                self.paths_with_cheating.push(path_to_now);
            }
            return;
        }

        for action in all_actions() {
            let next = self.execute_action(current_state, action);
            if next.is_none() { continue; }
            let next = next.unwrap();
            if self.states_handled.contains(&next) { continue; }
            let mut new_path = path_to_now.clone();
            new_path.push(action);
            self.continue_path(next, new_path);
        }
    }

    fn create_all_paths(&mut self) {
        let start_state = self.get_start_state();
        let path_to_now = Vec::new();
        self.continue_path(start_state, path_to_now);
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
    assert_eq!(cost_of_path(&puzzle.path_without_cheating.unwrap()), 84);


}

