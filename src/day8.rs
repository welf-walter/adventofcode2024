use std::collections::HashSet;
use std::ops::Range;

use crate::maps::Position;
use crate::maps::Area;

#[derive(Debug, PartialEq, Copy, Clone)]
struct Antenna {
    frequency:char,
    position:Position
}

struct Map {
    area:Area,
    antennas:Vec<Antenna>
}

impl Map {
    // mirror a at b with the given factors
    // only return if contained in map
    fn mirror(&self, a:Position,b:Position, factors:Range<i32>) -> Vec<Position> {
        let mut mirrored = Vec::new();
        for fac in factors {
            let x = b.0 as i32 + fac * (b.0 as i32 - a.0 as i32);
            let y = b.1 as i32 + fac * (b.1 as i32 - a.1 as i32);
            if self.area.contains_signed(x,y) {
                mirrored.push((x as usize,y as usize));
            }
        }
        mirrored
    }
}

fn parse_map(lines:&Vec<String>) -> Map {
    let mut antennas:Vec<Antenna> = Vec::new();
    let area = Area{width:lines[0].len(), height: lines.len()};
    for y in 0..area.height {
        let mut x = 0;
        for c in lines[y as usize].chars() {
            if c != '.' {
                antennas.push(Antenna{frequency:c, position:(x,y)});
            }
            x += 1;
        }
    }
    Map { area, antennas }
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
    assert_eq!(map.area.width, 12);
    assert_eq!(map.area.height, 12);
    assert_eq!(map.antennas.len(), 7);
    assert_eq!(map.antennas[4], Antenna{frequency:'A', position:(6,5)});
}

fn determine_antinodes(map:&Map, factors:Range<i32>) -> HashSet<Position> {
    let mut antinodes = HashSet::new();
    let len = map.antennas.len();
    for i in 0..len {
        let a = map.antennas[i];
        for j in i+1..len {
            let b = map.antennas[j];
            if a.frequency == b.frequency {
                for antinode in map.mirror(a.position,b.position,factors.clone()) {
                    antinodes.insert(antinode);
                };
                for antinode in map.mirror(b.position,a.position,factors.clone()) {
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
    let factors1 = 1..2;
    let antinodes1 = determine_antinodes(&map,factors1);
    assert!(antinodes1.contains(&(3,1)));
    assert!(!antinodes1.contains(&(10,9)));
    assert!(antinodes1.contains(&(10,10)));
    assert!(antinodes1.contains(&(10,11)));
    assert_eq!(antinodes1.len(), 14);
    let factors2 = 0..100;
    let antinodes1 = determine_antinodes(&map,factors2);
    assert!(antinodes1.contains(&(10,10)));
    assert!(!antinodes1.contains(&(11,10)));
    assert!(antinodes1.contains(&(10,11)));
    assert!(antinodes1.contains(&(11,11)));
    assert_eq!(antinodes1.len(), 34);
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
    let factors1 = 1..2;
    let antinodes1 = determine_antinodes(&map, factors1);
    println!("Day 8, Part 1: Map contains {} antennas and {} antinodes", map.antennas.len(), antinodes1.len());

    let factors2 = 0..100;
    let antinodes2 = determine_antinodes(&map, factors2);
    println!("Day 8, Part 2: Map contains {} antennas and {} antinodes", map.antennas.len(), antinodes2.len());

}
