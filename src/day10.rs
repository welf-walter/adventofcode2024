use crate::maps::PixelMap;

struct Height {
    value:u32;
}

impl crate::maps::CharBijection for Height {
    fn from_char(c:char) -> Self {
        match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            _ => panic!("Unexpected character {} for Height", c)
        }
    }
}

type Map = PixelMap<Height>;

// how many 
fn reachable_peaks(map:&Map, start_position:Position) -> HashSet<Position> {
    if map.at(start_position) == 9 {
        return HashSet::from([start_position]);
    }

    let mut peaks:HashSet<Position> = HashSet::new();
    for direction in [LEFT, UP, RIGHT, DOWN] {
        if let Some(next_position) = map.area.step(direction) {
            if map.at(next_position) = map.at(start_position) + 1 {
                let new_peaks = reachable_peaks(map, next_position);
                peaks = peaks.union(new_peaks);
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
    assert_eq!(reachable_peaks().collect(), vec![(0,4)]);
}