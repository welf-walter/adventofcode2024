use crate::maps::Position;
use crate::maps::Direction;
use crate::optimize::get_cost_cache;
use crate::optimize::get_cost_of_state;
use crate::optimize::Problem;
use Direction::*;

use std::collections::HashMap;

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
}

type Cost = u32;

struct ShortestPathProblem<'a> {
    map:&'a Map,
    start:Position,
    end:Position
}

impl<'a> Problem for ShortestPathProblem<'a> {

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

    fn cost(&self, _action:Self::Action) -> crate::optimize::Cost {
        1
    }

    fn all_actions(&self) -> Vec<Self::Action> {
        vec![
            Up, Right, Down, Left
        ]
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
    let problem = ShortestPathProblem{map, start, end};
    get_cost_of_state(&problem, problem.start)
}

struct Puzzle {
    // todo: we could reference an existing map
    map:Map,
    cost_map:HashMap<Position,Cost>,
    cost_of_path_without_cheating:Cost  // not really required!
}

type Cheat = (/*from:*/Position, /*to:*/Position, /*cost_for_cheat:*/Cost);

impl Puzzle {
    fn from<'a>(map_lines:impl Iterator<Item=&'a str>) -> Puzzle {
        let map = Map::from_strings(map_lines);
        let cost_of_path_without_cheating = cost_of_shortest_path(&map, map.find_first(Start).unwrap(), map.find_first(End).unwrap());
        Puzzle {
            map,
            cost_map:HashMap::new(),
            cost_of_path_without_cheating
        }
    }

    fn create_cost_map(&mut self) {
        let reverse_start = self.map.find_first(End).unwrap();
        let reverse_end = self.map.find_first(Start).unwrap();

        let problem = ShortestPathProblem{map:&self.map, start:reverse_start, end:reverse_end};
        self.cost_map = get_cost_cache(&problem, reverse_start);

        if VERBOSE {
            for (pos, cost) in &self.cost_map {
                println!("({},{})->{}", pos.0, pos.1, cost);
            }
        }

        assert_eq!(self.cost_of_path_without_cheating, *self.cost_map.get(&self.map.find_first(Start).unwrap()).unwrap());
    }

    fn get_all_cheats_part1(&self) -> Vec<Cheat> {
        const COST_OF_CHEAT : Cost = 2;

        let mut cheats:Vec<Cheat> = Vec::new();
        for start_state in self.map.area.all_positions() {
            // only iterate through two direction because the cheat is indirectional
            for direction in [Right, Down] {
                let after = self.execute_cheat(start_state, direction);
                if let Some(end_state) = after {
                    if VERBOSE { println!("  Cheat from ({},{}) to ({}, {})", start_state.0, start_state.1, end_state.0, end_state.1 );}
                    cheats.push((start_state, end_state, COST_OF_CHEAT));
                }
            }
        }
        cheats
    }

    const CHEAT_MAX_LEN_PART_2:u32 = 20;

    fn get_all_cheats_part2(&self) -> Vec<Cheat> {
        let mut cheats:Vec<Cheat> = Vec::new();
        for start_state in self.map.area.all_positions() {
            if self.map.at(start_state) == Wall { continue; }
            for length in 2..Self::CHEAT_MAX_LEN_PART_2+1 {
                // for length 3 we want to have [(-3,0),(-2,1),(-1,2),(0,3),(1,2),(2,1)] - because (-3,0) == (3,0)
                for dx in -(length as i32)..(length as i32) {
                    let dy = (length as i32) - dx.abs();
                    let end_state = self.map.area.position_add(start_state, dx, dy);
                    if end_state.is_none() { continue;}
                    let end_state = end_state.unwrap();
                    if self.map.at(end_state) == Wall { continue; }
                    if VERBOSE { println!("  Cheat from ({},{}) to ({}, {}) with length {}", start_state.0, start_state.1, end_state.0, end_state.1, length );}
                    cheats.push((start_state, end_state, length));
                }
            }
        }
        cheats
    }

    fn get_savings_of_cheats(&self, cheats:&Vec<Cheat>, minimum_saving:Cost) -> Vec<Cost> {
        cheats.iter().map(
            |cheat|
            {
                let cost_of_shortest_path =
                    self.cost_map.get(&cheat.0).unwrap().abs_diff(
                        *self.cost_map.get(&cheat.1).unwrap()
                    );
                let saving = cost_of_shortest_path - cheat.2;
                if VERBOSE { println!("  Cheat from ({},{}) to ({}, {}) with length {}: Saving = {}", cheat.0.0, cheat.0.1, cheat.1.0, cheat.1.1, cheat.2, saving );}
                saving
            }
        ).filter(|&saving| saving >= minimum_saving)
        .collect()
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
    let puzzle = {
        let mut p = Puzzle::from(input.split('\n'));
        p.create_cost_map();
        p
    };
    let start_pos = puzzle.map.find_first(Start).unwrap();
    let problem = ShortestPathProblem{
        map: &puzzle.map,
        start: start_pos,
        end: puzzle.map.find_first(End).unwrap()
    };
    assert_eq!(problem.execute_action(start_pos, Up), Some((1,2)));
    assert_eq!(problem.execute_action(start_pos, Right), None);
    assert_eq!(problem.execute_action(start_pos, Left), None);
    assert_eq!(puzzle.execute_cheat(start_pos, Right), Some((3,3)));
    assert_eq!(puzzle.execute_cheat(start_pos, Left), None);

    assert_eq!(puzzle.cost_of_path_without_cheating, 84);

    let all_cheats1 = puzzle.get_all_cheats_part1();
    assert_eq!(all_cheats1.len(), 14+14+2+4+2+3+5);

    let mut path_savings1 = puzzle.get_savings_of_cheats(&all_cheats1, 0);
    path_savings1.sort();
    assert_eq!(path_savings1, vec![
        2,2,2,2,2,2,2,2,2,2,2,2,2,2,
        4,4,4,4,4,4,4,4,4,4,4,4,4,4,
        6,6,
        8,8,8,8,
        10,10,
        12,12,12,
        20, 36, 38, 40, 64
    ]);

    let mut path_savings1b = puzzle.get_savings_of_cheats(&all_cheats1, 20);
    path_savings1b.sort();
    assert_eq!(path_savings1b, vec![
        20, 36, 38, 40, 64
    ]);

    let all_cheats2 = puzzle.get_all_cheats_part2();
    assert!(all_cheats2.len() > 32+31+29+39+25+23+20+19+12+14+12+22+4+3);

    let path_savings2 = puzzle.get_savings_of_cheats(&all_cheats2, 50);
    assert_eq!(path_savings2.len(), 32+31+29+39+25+23+20+19+12+14+12+22+4+3);

}

//////////////////////////////////////////
/// Puzzle
//////////////////////////////////////////

pub fn puzzle() {
    let lines = crate::helper::read_file("input/day20.txt");

    let mut puzzle = Puzzle::from(lines.iter().map(|line| line.as_str()));
    if VERBOSE { println!("Day 20: Full path is {} picoseconds", puzzle.cost_of_path_without_cheating)}
    puzzle.create_cost_map();
    let all_cheats1 = puzzle.get_all_cheats_part1();
    println!("Number of cheats (length=2) is {}", all_cheats1.len());
    let path_savings1 = puzzle.get_savings_of_cheats(&all_cheats1, 100);

    println!("Day 20, Part 1: Number of cheats (length=2) saving at least 100 picoseconds is {}", path_savings1.len());

    let all_cheats2 = puzzle.get_all_cheats_part2();
    println!("Number of cheats (length<=20) is {}", all_cheats2.len());
    let path_savings2 = puzzle.get_savings_of_cheats(&all_cheats2, 100);

    println!("Day 20, Part 2: Number of cheats (length<=20) saving at least 100 picoseconds is {}", path_savings2.len());
}
