
use std::char;

use crate::maps::Position;

// 0 1 2 3 4 5 6 7 8 9 A
type NumericKey = char;

const NUMERIC_KEY_START : Position = (2,3);

fn numeric_key_to_position(numeric_key:NumericKey) -> Position {
    match numeric_key {
        '7' => (0,0),
        '8' => (1,0),
        '9' => (2,0),
        '4' => (0,1),
        '5' => (1,1),
        '6' => (2,1),
        '1' => (0,2),
        '2' => (1,2),
        '3' => (2,2),

        '0' => (1,3),
        'A' => (2,3),
        other => panic!("Unexpected numeric key {}", other)
    }
}

// ^ < v > A
type DirectionKey = char;

const DIRECTION_KEY_START : Position = (2,0);

fn positions_to_keys(from:Position, to:Position) -> Vec<DirectionKey> {
    let mut keys = Vec::new();
    let mut current = from;
    while current.0 < to.0 { keys.push('>'); current.0 += 1;}
    while current.1 < to.1 { keys.push('v'); current.1 += 1;}
    while current.0 > to.0 { keys.push('<'); current.0 -= 1;}
    while current.1 > to.1 { keys.push('^'); current.1 -= 1;}
    keys.push('A');
    keys
}

fn numeric_keys_to_direction_keys(numeric_keys:&Vec<NumericKey>) -> Vec<DirectionKey> {
    let mut keys = Vec::new();
    let mut pos = NUMERIC_KEY_START;
    for &numeric_key in numeric_keys {
        let to = numeric_key_to_position(numeric_key);
        keys.append(&mut positions_to_keys(pos, to));
        pos = to;
    }
    keys
}

#[test]
fn test() {
    let numeric_keys = "029A".chars().collect::<Vec<char>>();
    let direction_keys = numeric_keys_to_direction_keys(&numeric_keys);
    assert_eq!(direction_keys, "<A^A>^^AvvvA".chars().collect::<Vec<char>>());
}