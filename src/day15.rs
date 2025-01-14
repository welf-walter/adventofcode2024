use crate::maps::Position;
use crate::maps::Direction;
use crate::maps::FromChar;
use crate::maps::PixelMap;

#[cfg(test)]
use crate::helper::split_input_sections;
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

fn other_side_of_box(myself:MapElement, pos:Position) -> Position {
    match myself {
        BoxLeft  => (pos.0 + 1, pos.1),
        BoxRight => (pos.0 - 1, pos.1),
        _ => unreachable!()
    }
}

fn can_move_to(map:&Map, target_pos:Position, direction:Direction) -> bool {
    let target = map.at(target_pos);
    match target {
        Wall => { return false },
        Space => { return true },
        Box | BoxLeft | BoxRight => { return can_move_box(map, target_pos, direction) },
        other => { panic!("Unexpected {:?} at {:?}", other, target_pos)}
    }
}

fn can_move_box(map:&Map, pos:Position, direction:Direction) -> bool {
    let myself = map.at(pos);
    if VERBOSE { println!("  Can I move {:?} at {:?} to {:?}?", myself, pos, direction)}
    assert!(myself == Box || myself == BoxLeft || myself == BoxRight );
    return match (myself, direction) {
        (Box, _) => can_move_to(map, map.area.step(pos, direction).unwrap(), direction),
        // @[]
        (BoxLeft, Right) |
        (BoxRight, Left) => can_move_to(map, map.area.step(map.area.step(pos, direction).unwrap(), direction).unwrap(), direction),
        (_, Up) |
        (_, Down) => can_move_to(map, map.area.step(pos, direction).unwrap(), direction)
                  && can_move_to(map, map.area.step(other_side_of_box(myself, pos), direction).unwrap(), direction),
        other => { panic!("Unexpected {:?} at {:?}", other, pos)}
    }
}

fn move_to(map:&mut Map, target_pos:Position, direction:Direction) -> u32 {
    let target = map.at(target_pos);
    return match target {
        Space => { 0 },
        Box | BoxLeft | BoxRight => { move_box(map, target_pos, direction) },
        other => { panic!("Unexpected {:?} at {:?}", other, target_pos)}
    }
}

// return number of boxes
fn move_box(map:&mut Map, pos:Position, direction:Direction) -> u32 {
    let myself = map.at(pos);
    if VERBOSE { println!("  Move {:?} at {:?} to {:?}!", myself, pos, direction)}
    assert!(myself == Box || myself == BoxLeft || myself == BoxRight );
    assert!(can_move_box(map, pos, direction));
    // make place for the box
    let moved_behind = match (myself, direction) {
        (Box, _) => { move_to(map, map.area.step(pos, direction).unwrap(), direction) },
        // @[]
        (BoxLeft, Right) |
        (BoxRight, Left) => { move_to(map, map.area.step(map.area.step(pos, direction).unwrap(), direction).unwrap(), direction) },
        (_, Up) |
        (_, Down) => {
            move_to(map, map.area.step(pos, direction).unwrap(), direction);
            move_to(map, map.area.step(other_side_of_box(myself, pos), direction).unwrap(), direction)
        },
        other => { panic!("Unexpected {:?} at {:?}", other, pos)}
    };

    let behind_pos = match (myself, direction) {
        (BoxRight, Left) |  // []@
        (BoxLeft, Right) => // @[]
            map.area.step(map.area.step(pos, direction).unwrap(), direction).unwrap(),
        _ => map.area.step(pos, direction).unwrap()
    };
    match myself {
        Box => {
            map.set_at(behind_pos, Box);
            map.set_at(pos, Space);
        },
        BoxLeft | BoxRight => {
            let pos_other = other_side_of_box(myself, pos);
            let new_pos = map.area.step(pos, direction).unwrap();
            let new_pos_other = map.area.step(pos_other, direction).unwrap();
            let other = match myself {
                BoxLeft => BoxRight,
                BoxRight => BoxLeft,
                _ => unreachable!()
            };
            map.set_at(pos, Space);
            map.set_at(pos_other, Space);
            map.set_at(new_pos, myself);
            map.set_at(new_pos_other, other);
        },
        _ => unreachable!()
    };
    moved_behind + 1

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
            Box | BoxLeft | BoxRight   => {
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
        if VERBOSE {map.println();}
    }
    map
}

fn get_gps(map:&Map) -> usize {
    let mut gps = 0;
    for pos in map.area.all_positions() {
        let pixel = map.at(pos);
        if  pixel == Box || pixel == BoxLeft {
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
    let puzzle1 = read_input(sections[0].split('\n'), &sections[1].replace("\n",""));
    let final_map1 = execute_moves(&puzzle1);
    if VERBOSE { final_map1.println(); }
    assert_eq!(get_gps(&final_map1), 10092);

    let puzzle2 = convert_to_part2(&puzzle1);
    if VERBOSE { puzzle2.map.println(); }
    let final_map2 = execute_moves(&puzzle2);
    if VERBOSE { final_map2.println(); }
    assert_eq!(get_gps(&final_map2), 9021);

}

#[cfg(test)]
fn input3() -> &'static str {
"#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^"
}

#[test]
fn test_puzzle3()
{
    let sections = split_input_sections::<2>(input3());
    let puzzle1 = read_input(sections[0].split('\n'), &sections[1].replace("\n",""));
    let puzzle2 = convert_to_part2(&puzzle1);
    if VERBOSE { puzzle2.map.println(); }
    let final_map = execute_moves(&puzzle2);
    if VERBOSE { final_map.println(); }
    assert_eq!(get_gps(&final_map), 105 + 207 + 306);
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
