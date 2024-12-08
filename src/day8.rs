use std::collections::HashSet;
use std::ops::Range;

type Position = (/* x: */i32,/* y: */i32);

#[derive(Debug, PartialEq, Copy, Clone)]
struct Antenna {
    frequency:char,
    position:Position
}

struct Map {
    width:i32,
    height:i32,
    antennas:Vec<Antenna>
}

impl Map {
    fn contains(&self, pos:Position) -> bool {
        pos.0 >= 0 && pos.1 >= 0 && pos.0 < self.width && pos.1 < self.height
    }

    // mirror a at b with the given factors
    // only return if contained in map
    fn mirror(&self, a:Position,b:Position, factors:Range<i32>) -> Vec<Position> {
        let mut mirrored = Vec::new();
        for fac in factors {
            let c = ( b.0 + fac * (b.0-a.0),
                      b.1 + fac * (b.1-a.1));
            if self.contains(c) {
                mirrored.push(c);
            }
        }
        mirrored
    }
}

fn parse_map(lines:&Vec<String>) -> Map {
    let mut antennas:Vec<Antenna> = Vec::new();
    let width = lines[0].len() as i32;
    let height = lines.len() as i32;
    for y in 0..height {
        let mut x = 0;
        for c in lines[y as usize].chars() {
            if c != '.' {
                antennas.push(Antenna{frequency:c, position:(x as i32,y as i32)});
            }
            x += 1;
        }
    }
    Map { width, height, antennas }
}

#[cfg(test)]
fn input1() -> Vec<String> {
"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............".split('\n').map(|line| line.to_string()).collect()
}

#[test]
fn test_parse() {
    let map = parse_map(&input1());
    assert_eq!(map.width, 12);
    assert_eq!(map.height, 12);
    assert_eq!(map.antennas.len(), 7);
    assert_eq!(map.antennas[4], Antenna{frequency:'A', position:(6,5)});
}

fn determine_antinodes(map:&Map) -> HashSet<Position> {
    let mut antinodes = HashSet::new();
    let len = map.antennas.len();
    for i in 0..len {
        let a = map.antennas[i];
        for j in i+1..len {
            let b = map.antennas[j];
            if a.frequency == b.frequency {
                for antinode in map.mirror(a.position,b.position,1..2) {
                    antinodes.insert(antinode);
                };
                for antinode in map.mirror(b.position,a.position,1..2) {
                    antinodes.insert(antinode);
                };
            }

        }
    }
    antinodes
}

#[test]
fn test_determine_antinodes() {
    let map = parse_map(&input1());
    let antinodes = determine_antinodes(&map);
    assert!(antinodes.contains(&(3,1)));
    assert!(!antinodes.contains(&(10,9)));
    assert!(antinodes.contains(&(10,10)));
    assert!(antinodes.contains(&(10,11)));
    assert_eq!(antinodes.len(), 14);
}

//////////////////////////////////////////
/// Puzzle
//////////////////////////////////////////

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;


pub fn puzzle() {
    let file = File::open("input/day8.txt").expect("Could not open input/day8.txt");
    let reader = BufReader::new(file);

    let lines:Vec<String> = reader.lines().map( |line| line.unwrap() ).collect();

    let map = parse_map(&lines);
    let antinodes = determine_antinodes(&map);
    println!("Day 8, Part 1: Map contains {} antennas and {} antinodes", map.antennas.len(), antinodes.len());

}
