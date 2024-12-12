use crate::maps::Position;
use crate::maps::Direction;

type Plant = char;
type PlantMap = crate::maps::PixelMap<Plant>;

#[derive(Debug, PartialEq)]
struct Region {
    plant:Plant,
    area:u32, // = number of adjacent pixels
    perimeter:u32 // = number of different neighbours
}

// plant already handled. ignore it.
const DONE:Plant = '.';

fn extract_region(map:&mut PlantMap, start_position:Position) -> Region {
    let mut position_backlog = vec![start_position];
    let plant = map.at(start_position);
    let mut area = 0;
    let mut perimeter = 0;
    while position_backlog.len() > 0 {
        let current_pos = position_backlog.pop().unwrap();
        let current_plant = map.at(current_pos);
        assert!(current_plant != DONE);
        map.set_at(current_pos, DONE);
        if current_plant == plant {
            area += 1;
            for direction in Direction::all_directions() {
                match  map.area.step(current_pos, direction) {
                    Some(next_pos) => {
                        if map.at(next_pos) != DONE {
                            position_backlog.push(next_pos);
                        }
                    },
                    None => {
                        perimeter += 1;
                    }
                }
            }
        } else {
            perimeter += 1;
        }
    }

    Region { plant, area, perimeter }
}

fn extract_regions(map:&PlantMap) -> Vec<Region> {
    let mut regions = Vec::new();
    let mut workmap:PlantMap = map.clone();
    for pos in map.area.all_positions() {
        if workmap.at(pos) != DONE {
            regions.push(extract_region(&mut workmap, pos));
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