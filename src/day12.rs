use std::collections::HashSet;

use crate::maps::Position;
use crate::maps::Direction;

const VERBOSE:bool = true;

type Plant = char;
type PlantMap = crate::maps::PixelMap<Plant>;

#[derive(Debug, PartialEq)]
struct Region {
    plant:Plant,
    area:u32, // = number of adjacent pixels
    perimeter:u32 // = number of different neighbours
}

fn extract_region(map:&PlantMap, start_position:Position, positions_done:&mut HashSet<Position>) -> Region {
    let mut position_backlog = vec![start_position];
    let plant = map.at(start_position);
    if VERBOSE { println!("extract region {}", plant);}
    let mut area = 0;
    let mut perimeter = 0;
    while position_backlog.len() > 0 {
        let current_pos = position_backlog.pop().unwrap();

        let current_plant = map.at(current_pos);
        assert_eq!(current_plant, plant);
        if VERBOSE { print!("  ({},{}): {} -> ", current_pos.0, current_pos.1, current_plant);}

        if positions_done.contains(&current_pos) {
            if VERBOSE { println!("  ({},{}): been there. done that.", current_pos.0, current_pos.1);}
            continue;
        }
        positions_done.insert(current_pos);
        area += 1;
        for direction in Direction::all_directions() {
            match map.area.step(current_pos, direction) {
                Some(next_pos) => {
                    let next_plant = map.at(next_pos);
                    if next_plant == current_plant {
                        if !positions_done.contains(&next_pos) {
                            position_backlog.push(next_pos);
                            if VERBOSE { print!("Put ({},{}) to backlog ", next_pos.0, next_pos.1);}
                        }
                    } else {
                        perimeter += 1;
                        if VERBOSE { println!("{:?} = perimeter", direction);}
                    }
                },
                None => {
                    perimeter += 1;
                    if VERBOSE { print!("{:?} = border ", direction);}
                }
            }
        }
        if VERBOSE { println!("");}

    }

    Region { plant, area, perimeter }
}

fn extract_regions(map:&PlantMap) -> Vec<Region> {
    let mut regions = Vec::new();
    let mut positions_done = HashSet::new();
    for pos in map.area.all_positions() {
        if !positions_done.contains(&pos) {
            regions.push(extract_region(&map, pos, &mut positions_done));
        }
    }
    regions
}

#[test]
fn test_region() {
    let input =
"AAAA
BBCD
BBCC
EEEC";
    let map = PlantMap::from_strings(input.split('\n'));
    assert_eq!(map.at((2,1)), 'C');
    let regions = extract_regions(&map);
    assert_eq!(regions, vec![
        Region{plant:'A', area: 4, perimeter: 10},
        Region{plant:'B', area: 4, perimeter: 8},
        Region{plant:'C', area: 4, perimeter: 10},
        Region{plant:'D', area: 1, perimeter: 4},
        Region{plant:'E', area: 3, perimeter: 8}
    ]);
}