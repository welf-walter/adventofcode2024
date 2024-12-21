use crate::maps::Position;
use crate::maps::Direction;
use crate::optimize::get_cost_of_state;
use crate::optimize::Problem;
use Direction::*;

const VERBOSE:bool = false;

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



use crate::optimize::ActionTrait;

impl ActionTrait for Direction {
    fn all_actions() -> &'static [Self] {
        &[
            Up, Right, Down, Left
        ]
    }

    fn cost(self) -> crate::optimize::Cost {
        1
    }
}

type Cost = u32;

const COST_OF_CHEAT : Cost = 2;

struct ShortestPathProblem {
    map:Map,
    start:Position,
    end:Position
}

impl Problem for ShortestPathProblem {

    type State = Position;
    type Action = Direction;

    fn is_end_state(&self, state:&Self::State) -> bool {
        *state == self.end
    }

    // none means action cannot be executed
    fn execute_action(&self, before:Position, action:Direction) -> Option<Position> {
        let after = self.map.area.step(before, action);
        if after.is_none() { return None; }
        let after = after.unwrap();
        if self.map.at(after) == Wall { return None; }
        return Some(after);
    }

}

impl Puzzle {

    fn execute_cheat(&self, before:Position, action:Direction) -> Option<Position> {
        if self.map.at(before) == Wall { return None; }
        let after1 = self.map.area.step(before, action);
        if after1.is_none() { return None; }
        let after1 = after1.unwrap();
        // if there is no wall we could have done this without cheating
        if self.map.at(after1) != Wall { return None; }
        let after2 = self.map.area.step(after1, action);
        if after2.is_none() { return None; }
        let after2 = after2.unwrap();
        if self.map.at(after2) == Wall { return None; }
        return Some(after2);
    }

}

fn cost_of_shortest_path(map:&Map, start:Position, end:Position) -> Cost {
    let problem = ShortestPathProblem{map:map.clone(), start, end};
    get_cost_of_state(&problem, problem.start)
}

struct Puzzle {
    // todo: we could reference an existing map
    map:Map,
    cost_of_path_without_cheating:Cost  // not really required!
}

impl Puzzle {
    fn from<'a>(map_lines:impl Iterator<Item=&'a str>) -> Puzzle {
        let map = Map::from_strings(map_lines);
        let cost_of_path_without_cheating = cost_of_shortest_path(&map, map.find_first(Start).unwrap(), map.find_first(End).unwrap());
        Puzzle {
            map,
            cost_of_path_without_cheating
        }
    }

    fn get_all_cheats(&self) -> Vec<(Position, Position)> {
        let mut cheats:Vec<(Position, Position)> = Vec::new();
        for start_state in self.map.area.all_positions() {
            // only iterate through two direction because the cheat is indirectional
            for direction in [Right, Down] {
                let after = self.execute_cheat(start_state, direction);
                if let Some(end_state) = after {
                    if VERBOSE { println!("  Cheat from ({},{}) to ({}, {})", start_state.0, start_state.1, end_state.0, end_state.1 );}
                    cheats.push((start_state, end_state));
                }
            }
        }
        cheats
    }

    fn get_savings_of_cheats(&self, cheats:&Vec<(Position, Position)>) -> Vec<Cost> {
        cheats.iter().map(
            |cheat|
            cost_of_shortest_path(&self.map, cheat.0, cheat.1) - COST_OF_CHEAT
        ).collect()
    }
/*
    fn continue_path(&mut self, current_state:Position, path_to_now:&mut Path, been_there:&mut HashSet<Position>) {
        if VERBOSE { println!("At ({},{}), {} cheats left", current_state.0.0, current_state.0.1, current_state.1);}

        if self.map.at(current_state.0) == End {
            if current_state.1 > 0 {
                assert!(self.path_without_cheating.is_none());
                if VERBOSE { println!("  Finished without cheating, cost = {}", cost_of_path(path_to_now));}
                self.path_without_cheating = Some(path_to_now.clone());
            } else {
                if VERBOSE { println!("  Finished with cheating, cost = {}", cost_of_path(path_to_now));}
                self.paths_with_cheating.push(path_to_now.clone());
            }
            return;
        }

        assert!(!been_there.contains(&current_state.0));
        been_there.insert(current_state.0);

        for action in all_actions() {
            if VERBOSE { println!("  Try to do {:?}", action);}
            let next = self.execute_action(current_state, action);
            if next.is_none() { continue; }
            let next = next.unwrap();
            if been_there.contains(&next.0) {
                if VERBOSE { println!("  Been there. Done that.");}
                continue;
            }
            path_to_now.push(action);
            let level = path_to_now.len();
            if VERBOSE { println!("  Recurse at level {}", level);}
            self.continue_path(next, path_to_now, been_there);
            if VERBOSE { println!("  Back from level {} on ({},{}), {} cheats left", level, current_state.0.0, current_state.0.1, current_state.1);}
            let a = path_to_now.pop();
            assert_eq!(a.unwrap(), action);
        }

        assert!(been_there.contains(&current_state.0));
        been_there.remove(&current_state.0);

        if VERBOSE { println!("  Done with ({},{}), {} cheats left", current_state.0.0, current_state.0.1, current_state.1);}

    }

    fn create_all_paths(&mut self) {
        let start_state = self.get_start_state();
        let mut path_to_now = Vec::new();
        let mut been_there = HashSet::new();
        self.continue_path(start_state, & mut path_to_now, &mut been_there);
    }

    fn get_cheating_path_savings(&self) -> Vec<Cost> {
        let original_cost = cost_of_path(self.path_without_cheating.as_ref().unwrap());
        self.paths_with_cheating.iter().map(|path| original_cost - cost_of_path(path)).collect::<Vec<Cost>>()
    }
*/



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
    let puzzle = Puzzle::from(input.split('\n'));
    let start_pos = puzzle.map.find_first(Start).unwrap();
    let problem = ShortestPathProblem{
        map: puzzle.map.clone(),
        start: start_pos,
        end: puzzle.map.find_first(End).unwrap()
    };
    assert_eq!(problem.execute_action(start_pos, Up), Some((1,2)));
    assert_eq!(problem.execute_action(start_pos, Right), None);
    assert_eq!(problem.execute_action(start_pos, Left), None);
    assert_eq!(puzzle.execute_cheat(start_pos, Right), Some((3,3)));
    assert_eq!(puzzle.execute_cheat(start_pos, Left), None);

    assert_eq!(puzzle.cost_of_path_without_cheating, 84);
    let all_cheats = puzzle.get_all_cheats();
    assert_eq!(all_cheats.len(), 14+14+2+4+2+3+5);

    let mut path_savings = puzzle.get_savings_of_cheats(&all_cheats);
    path_savings.sort();
    assert_eq!(path_savings, vec![
        2,2,2,2,2,2,2,2,2,2,2,2,2,2,
        4,4,4,4,4,4,4,4,4,4,4,4,4,4,
        6,6,
        8,8,8,8,
        10,10,
        12,12,12,
        20, 36, 38, 40, 64
    ]);

}

//////////////////////////////////////////
/// Puzzle
//////////////////////////////////////////

pub fn puzzle() {
    let lines = crate::helper::read_file("input/day20.txt");

    let puzzle = Puzzle::from(lines.iter().map(|line| line.as_str()));
    if VERBOSE { println!("Day 20: Full path is {} picoseconds", puzzle.cost_of_path_without_cheating)}
    let all_cheats = puzzle.get_all_cheats();
    let path_savings = puzzle.get_savings_of_cheats(&all_cheats);
    // why "&&saving"?
    let cheat_count = path_savings.iter().filter(|&&saving| saving >= 100).count();

    println!("Day 20, Part 1: Number of cheats saving at least 100 picoseconds is {}", cheat_count);
}
