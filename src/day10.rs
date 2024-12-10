use std::collections::HashSet;

use crate::maps::PixelMap;
use crate::maps::Position;
use crate::maps::Direction::*;

#[derive(Clone, Copy)]
struct Height {
    value:u32
}

impl crate::maps::CharBijection for Height {
    fn from_char(c:char) -> Self {
        Height { value: c.to_digit(10).unwrap() }
    }
    fn to_char(&self) -> char {
        char::from_digit(self.value, 10).unwrap()
    }
}

type Map = PixelMap<Height>;

// how many 
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

}