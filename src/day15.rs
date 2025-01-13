use crate::maps::Position;
use crate::maps::Direction;
use crate::maps::FromChar;
use crate::maps::PixelMap;

#[cfg(test)]
use crate::helper::split_input_sections;
#[cfg(test)]
use crate::maps::Direction::*;

const VERBOSE:bool = true;

#[derive(Debug, PartialEq, Clone, Copy)]
enum MapElement {
    Space,
    Wall,
    Box,
    BoxLeft,
    BoxRight,
    Robot,
    Unknown
}

use MapElement::*;


impl crate::maps::FromChar for MapElement {
    fn from_char(c:char) -> Self {
        match c {
            '.' => Space,
            '#' => Wall,
            'O' => Box,
            '[' => BoxLeft,
            ']' => BoxRight,
            '@' => Robot,
            _ => panic!("Unexpected character {} for TestEnum", c)
        }
    }
}

impl crate::maps::ToChar for MapElement {
    fn to_char(self) -> char {
        match self {
            Space => '.',
            Wall => '#',
            Box => 'O',
            BoxLeft => '[',
            BoxRight => ']',
            Robot => '@',
            Unknown => '?'
        }
    }
}

type Map = PixelMap<MapElement>;

struct Puzzle {
    map:Map,
    moves:Vec<Direction>
}

fn can_move_box(map:&Map, pos:Position, direction:Direction) -> bool {
    assert!(map.at(pos) == Box);
    let behind_pos = map.area.step(pos, direction).unwrap();
    match map.at(behind_pos) {
        Wall => { return false },
        Space => { return true },
        Box => { return can_move_box(map, behind_pos, direction) },
        other => { panic!("Unexpected {:?} at {:?}", other, behind_pos)}
    }
}

// return number of boxes
fn move_box(map:&mut Map, pos:Position, direction:Direction) -> u32 {
    assert!(map.at(pos) == Box);
    assert!(can_move_box(map, pos, direction));
    let behind_pos = map.area.step(pos, direction).unwrap();
    let box_count_behind =
    match map.at(behind_pos) {
        Space => { 0 },
        Box => { move_box(map, behind_pos, direction) },
        other => { panic!("Unexpected {:?} at {:?}", other, behind_pos)}
    };
    map.set_at(behind_pos, Box);
    map.set_at(pos, Space);
    box_count_behind + 1
}

fn read_input<'a>(map_lines:impl Iterator<Item=&'a str>, directions_lines:&str) -> Puzzle {
    let map = PixelMap::from_strings(map_lines);
    let moves = directions_lines.chars().map(|c| Direction::from_char(c)).collect();
    Puzzle { map, moves }
}

// extract robot start position and replace with Space
fn extract_start_pos(map:&mut Map) -> Position {
    if let Some(pos) = map.find_first(Robot) {
        map.set_at(pos, Space);
        return pos;
    } else {
        panic!("Could not find start position");
    }
}

fn convert_to_part2(puzzle:&Puzzle) -> Puzzle {
    let     map = &puzzle.map;
    let mut map2 = PixelMap::new(map.width() * 2, map.height(),Unknown);
    for pos in map.area.all_positions() {
        let (element1,element2) = match map.at(pos) {
            Wall  => (Wall, Wall),
            Box   => (BoxLeft, BoxRight),
            Space => (Space, Space),
            Robot => (Robot, Space),
            other     => { panic!("Unexpected {:?} at {:?}", other, pos) }
        };
        map2.set_at((pos.0 * 2    ,pos.1), element1);
        map2.set_at((pos.0 * 2 + 1,pos.1), element2);
    }
    Puzzle { map:map2, moves:puzzle.moves.clone() }
}

fn execute_moves(puzzle:&Puzzle) -> Map {
    let mut map = puzzle.map.clone();
    let mut current_pos = extract_start_pos(&mut map);
    for &direction in &puzzle.moves {
        let next_pos = map.area.step(current_pos, direction).unwrap();
        match map.at(next_pos) {
            Space => {
                if VERBOSE {println!("Move {:?} to {:?}", direction, next_pos);}
                current_pos = next_pos;
            }
            Wall  => {
                if VERBOSE {println!("Cannot move {:?}", direction);}
            }
            Box   => {
                if can_move_box(&map, next_pos, direction) {
                    let boxes_moved = move_box(&mut map, next_pos, direction);
                    current_pos = next_pos;
                    if VERBOSE {println!("Move {} boxes at {:?} {:?}", boxes_moved, next_pos, direction);}
                } else {
                    if VERBOSE {println!("Cannot move box {:?}", direction);}
                }
            },
            _ => unreachable!()
        }
    }
    map
}

fn get_gps(map:&Map) -> usize {
    let mut gps = 0;
    for pos in map.area.all_positions() {
        if map.at(pos) == Box {
            gps += pos.0 + pos.1 * 100;
        }
    }
    gps
}

#[cfg(test)]
fn input1() -> &'static str {
"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"
}

#[test]
fn test_puzzle1()
{
    let sections1 = split_input_sections::<2>(input1());
    let puzzle1 = read_input(sections1[0].split('\n'), &sections1[1]);
    assert_eq!(puzzle1.map.pixels[2], vec![Wall, Wall, Robot, Space, Box, Space, Space, Wall]);
    assert_eq!(puzzle1.moves[0..7], [Left, Up, Up, Right, Right, Right, Down]);
    let start_pos = extract_start_pos(&mut puzzle1.map.clone());
    assert_eq!(start_pos, (2,2));
    let final_map = execute_moves(&puzzle1);
    assert_eq!(get_gps(&final_map), 2028);

    let puzzle1_2 = convert_to_part2(&puzzle1);
    assert_eq!(puzzle1_2.map.pixels[2], vec![Wall, Wall, Wall, Wall, Robot, Space, Space, Space, BoxLeft, BoxRight, Space, Space, Space, Space, Wall, Wall]);
}

#[cfg(test)]
fn input2() -> &'static str {
"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
}

#[test]
fn test_puzzle2()
{
    let sections = split_input_sections::<2>(input2());
    let puzzle = read_input(sections[0].split('\n'), &sections[1].replace("\n",""));
    let final_map = execute_moves(&puzzle);
    if VERBOSE { final_map.println(); }
    assert_eq!(get_gps(&final_map), 10092);
}

//////////////////////////////////////////
/// Puzzle
//////////////////////////////////////////

use crate::helper::split_lines_sections;

pub fn puzzle() {
    let lines = crate::helper::read_file("input/day15.txt");

    let sections = split_lines_sections::<2>(lines);
    let puzzle = read_input(sections[0].iter().map(|line| line.as_str()), &sections[1].join(""));
    let final_map = execute_moves(&puzzle);
    let gps = get_gps(&final_map);

    println!("Day 15, Part 1: GPS after moving is {}", gps);

    let _puzzle2 = convert_to_part2(&puzzle);
}
