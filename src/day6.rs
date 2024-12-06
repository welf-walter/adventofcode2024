use std::collections::HashSet;

type Position = (/* x: */usize,/* y: */usize);

#[derive(Debug,Copy,Clone)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            UP    => RIGHT,
            RIGHT => DOWN,
            DOWN  => LEFT,
            LEFT  => UP
        }
    }
}

use Direction::*;

struct Map {
    width:usize,
    height:usize,
    obstructions:HashSet<Position>,
    start:Position // Direction is up
}

impl Map {
    // return None if out of area
    fn step(&self, pos:Position, direction:Direction) -> Option<Position> {
        match direction {
            UP    => { if pos.1 > 0             { return Some((pos.0  ,pos.1-1)); } else { return None; }},
            RIGHT => { if pos.0 < self.width-1  { return Some((pos.0+1,pos.1  )); } else { return None; }},
            DOWN  => { if pos.1 < self.height-1 { return Some((pos.0  ,pos.1+1)); } else { return None; }},
            LEFT  => { if pos.0 > 0             { return Some((pos.0-1,pos.1  )); } else { return None; }}
        }
    }
}

fn read_map(lines:&Vec<String>) -> Map {
    let height = lines.len();
    let width = lines[0].len();
    let mut obstructions:HashSet<Position> = HashSet::new();
    let mut start:Position = (999,999);
    for y in 0..height {
        let chars = lines[y].chars();
        let mut x = 0;
        for c in chars {
            match c {
                '.' => { },
                '#' => { obstructions.insert((x,y)); },
                '^' => { start = (x,y); },
                _ => panic!("Unexpected character '{}'", c)
            }
            x += 1;
        }
    }
    Map { width, height, obstructions, start }

}

#[cfg(test)]
fn input1() -> Vec<String> {
"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...".split('\n').map(|s| s.to_string()).collect()
}

#[test]
fn test_map()
{
    let map = read_map(&input1());
    assert_eq!(map.width, 10);
    assert_eq!(map.height, 10);
    assert_eq!(map.start, (4,6));
    assert_eq!(map.obstructions.len(), 8);
    assert!(map.obstructions.contains(&(2,3)));
    assert_eq!(map.step((3,4),UP),Some((3,3)));
    assert_eq!(map.step((3,0),UP),None);
}

fn walk(map:&Map) -> HashSet<Position> {
    let mut pos = map.start;
    let mut direction = Direction::UP;
    let mut positions:HashSet<Position> = HashSet::new();
    loop {
        positions.insert(pos);
        let new_poso = map.step(pos, direction);
        if new_poso.is_none() {
            return positions;
        }
        let new_pos = new_poso.unwrap();
        if map.obstructions.contains(&new_pos) {
            direction = direction.turn_right();
        }
        else
        {
            pos = new_pos;
        }
    }
}

#[test]
fn test_walk() {
    let map = read_map(&input1());
    let positions = walk(&map);
    assert!(positions.contains(&(2,4)));
    assert_eq!(positions.len(), 41);
}

//////////////////////////////////////////
/// Puzzle
//////////////////////////////////////////

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn puzzle() {
    let file = File::open("input/day6.txt").expect("Could not open input/day6.txt");
    let reader = BufReader::new(file);

    let lines:Vec<String> = reader.lines().map( |line| line.unwrap() ).collect();

    let map = read_map(&lines);
    let positions = walk(&map);

    println!("Day 6, Part 1: Guard was on {} unique positions", positions.len());

}