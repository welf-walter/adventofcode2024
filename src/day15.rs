use crate::maps::Position;
use crate::maps::Direction;
use crate::maps::Direction::*;
use crate::maps::FromChar;
use crate::maps::PixelMap;
use crate::helper::split_input_sections;

#[derive(Debug, PartialEq, Clone, Copy)]
enum MapElement {
    Space,
    Wall,
    Box,
    Robot
}

use MapElement::*;


impl crate::maps::FromChar for MapElement {
    fn from_char(c:char) -> Self {
        match c {
            '.' => Space,
            '#' => Wall,
            'O' => Box,
            '@' => Robot,
            _ => panic!("Unexpected character {} for TestEnum", c)
        }
    }
}

type Map = PixelMap<MapElement>;

struct Puzzle {
    map:Map,
    moves:Vec<Direction>
}

fn read_input<'a>(map_lines:impl Iterator<Item=&'a str>, directions_lines:&str) -> Puzzle {
    let map = PixelMap::from_strings(map_lines);
    let moves = directions_lines.chars().map(|c| Direction::from_char(c)).collect();
    Puzzle { map, moves }
}

// extract robot start position and replace with Space
fn extract_start_pos(map:&mut Map) -> Position {
    for pos in map.area.all_positions() {
        if map.at(pos) == Robot {
            map.set_at(pos, Space);
            return pos;
        }
    }
    panic!("Could not find start position");
}

fn execute_moves(puzzle:&Puzzle) -> Map {
    let mut map = puzzle.map.clone();
    // ...
    map
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
fn test_puzzle()
{
    let sections = split_input_sections::<2>(input1());
    let puzzle1 = read_input(sections[0].split('\n'), &sections[1]);
    assert_eq!(puzzle1.map.pixels[2], vec![Wall, Wall, Robot, Space, Box, Space, Space, Wall]);
    assert_eq!(puzzle1.moves[0..7], [Left, Up, Up, Right, Right, Right, Down]);
    let start_pos = extract_start_pos(&mut puzzle1.map.clone());
    assert_eq!(start_pos, (2,2));
}