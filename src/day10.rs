use std::collections::HashSet;

use crate::maps::PixelMap;
use crate::maps::Position;
use crate::maps::Direction::*;

#[derive(Clone, Copy)]
struct Height {
    value:u32
}

impl crate::maps::FromChar for Height {
    fn from_char(c:char) -> Self {
        Height { value: c.to_digit(10).unwrap() }
    }
}

type Map = PixelMap<Height>;

// list all peaks that can be reached from start_position
fn reachable_peaks(map:&Map, start_position:Position) -> HashSet<Position> {
    if map.at(start_position).value == 9 {
        return HashSet::from([start_position]);
    }

    let mut peaks:HashSet<Position> = HashSet::new();
    for direction in [LEFT, UP, RIGHT, DOWN] {
        if let Some(next_position) = map.area.step(start_position, direction) {
            if map.at(next_position).value == map.at(start_position).value + 1 {
                let new_peaks = reachable_peaks(map, next_position);
                peaks.extend(&new_peaks);
            }
        }
    }

    peaks
}

fn sum_of_trailhead_scores(map:&Map) -> usize {

    let is_trailhead = |position:&Position| {
        map.at(*position).value == 0
    };

    let score_of_trailhead = |trailhead:Position| {
        reachable_peaks(&map, trailhead).len()
    };

    map.area.all_positions().filter(is_trailhead).map(score_of_trailhead).sum()
}

// count all ways to the peak from start_position
fn count_ways_to_peak(map:&Map, start_position:Position) -> u32 {
    if map.at(start_position).value == 9 {
        return 1;
    }

    let mut counter = 0;
    for direction in [LEFT, UP, RIGHT, DOWN] {
        if let Some(next_position) = map.area.step(start_position, direction) {
            if map.at(next_position).value == map.at(start_position).value + 1 {
                counter += count_ways_to_peak(map, next_position);
            }
        }
    }

    counter
}

fn sum_of_trailhead_rating(map:&Map) -> u32 {

    let is_trailhead = |position:&Position| {
        map.at(*position).value == 0
    };

    let rating_of_trailhead = |trailhead:Position| {
        count_ways_to_peak(&map, trailhead)
    };

    map.area.all_positions().filter(is_trailhead).map(rating_of_trailhead).sum()
}



#[test]
fn test_trail() {
    let input1 = 
"0123
1234
8765
9876";
    let map1 = Map::from_strings(input1.split('\n'));
    assert_eq!(map1.width(), 4);
    assert_eq!(map1.height(), 4);
    assert_eq!(reachable_peaks(&map1, (0,3)), HashSet::from([(0,3)]));
    assert_eq!(reachable_peaks(&map1, (0,3)), HashSet::from([(0,3)]));
    assert_eq!(sum_of_trailhead_scores(&map1), 1);

    let input2 =
"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
    let map2 = Map::from_strings(input2.split('\n'));
    assert_eq!(map2.width(), 8);
    assert_eq!(map2.height(), 8);
    assert_eq!(reachable_peaks(&map2, (0,0)), HashSet::from([(1,0)]));
    assert_eq!(reachable_peaks(&map2, (2,0)).len(), 5);
    assert_eq!(reachable_peaks(&map2, (4,0)).len(), 6);
    assert_eq!(reachable_peaks(&map2, (4,2)).len(), 5);
    assert_eq!(reachable_peaks(&map2, (6,4)).len(), 3);
    assert_eq!(reachable_peaks(&map2, (2,5)).len(), 1);
    assert_eq!(reachable_peaks(&map2, (5,5)).len(), 3);
    assert_eq!(reachable_peaks(&map2, (0,6)).len(), 5);
    assert_eq!(reachable_peaks(&map2, (6,6)).len(), 3);
    assert_eq!(reachable_peaks(&map2, (1,7)).len(), 5);
    assert_eq!(sum_of_trailhead_scores(&map2), 36);
    assert_eq!(sum_of_trailhead_rating(&map2), 81);

    let input3 =
"012345
123456
234567
345678
426789
567892";
    let map3 = Map::from_strings(input3.split('\n'));
    assert_eq!(sum_of_trailhead_rating(&map3), 227);

}

//////////////////////////////////////////
/// Puzzle
//////////////////////////////////////////

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;


pub fn puzzle() {
    let file = File::open("input/day10.txt").expect("Could not open input/day10.txt");
    let reader = BufReader::new(file);

    let lines:Vec<String> = reader.lines().map( |line| line.unwrap() ).collect();
    let map = Map::from_strings(lines.iter().map( |line| line.as_str() ));

    println!("Day 10, Part 1: Sum of trailhead scores is {}", sum_of_trailhead_scores(&map));
    println!("Day 10, Part 2: Sum of trailhead rating is {}", sum_of_trailhead_rating(&map));

}
