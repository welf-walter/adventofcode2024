use crate::maps::Direction;
use crate::maps::Direction::*;
use crate::maps::FromChar;
use crate::maps::PixelMap;

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

struct Puzzle {
    map:PixelMap<MapElement>,
    moves:Vec<Direction>
}

fn read_input<'a>(map_lines:impl Iterator<Item=&'a str>, directions_lines:&str) -> Puzzle {
    let map = PixelMap::from_strings(map_lines);
    let moves = directions_lines.chars().map(|c| Direction::from_char(c)).collect();
    Puzzle { map, moves }
}

// split a string at empty lines
fn split_input_sections<const SECTION_COUNT:usize>(input:&str) -> [String;SECTION_COUNT] {
    let mut parts = input.split("\n\n");
    const EMPTY_STRING: std::string::String = String::new();
    let mut sections:[String;SECTION_COUNT] = [EMPTY_STRING;SECTION_COUNT];
    for i in 0..SECTION_COUNT {
        let part = parts.next().unwrap();
        sections[i] = String::from(part);
    }
    assert!(parts.next().is_none());
    sections
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
}