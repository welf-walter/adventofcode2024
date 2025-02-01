
use std::char;

use crate::maps::{left, right, up, down, Position};

// 0 1 2 3 4 5 6 7 8 9 A
type NumericKey = char;

const NUMERIC_KEY_START : Position = (2,3);
const NUMERIC_KEY_GAP   : Position = (0,3);

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
const DIRECTION_KEY_GAP   : Position = (0,0);

fn direction_key_to_position(direction_key:DirectionKey) -> Position {
    match direction_key {
        '^' => (1,0),
        'A' => (2,0),
        '<' => (0,1),
        'v' => (1,1),
        '>' => (2,1),
        other => panic!("Unexpected direction key {}", other)
    }
}

fn positions_to_keys(from:Position, to:Position, invalid_position:Position) -> Vec<DirectionKey> {
    let mut keys = Vec::new();
    let mut current = from;
    while current != to {
        while current.1 < to.1 && down(current) != invalid_position { keys.push('v'); current = down(current);}
        while current.0 < to.0                                           { keys.push('>'); current = right(current);}
        while current.0 > to.0 && left(current) != invalid_position { keys.push('<'); current = left(current);}
        while current.1 > to.1 && up  (current) != invalid_position { keys.push('^'); current = up(current);}
    }
    keys.push('A');
    keys
}

fn numeric_keys_to_direction_keys(numeric_keys:&Vec<NumericKey>) -> Vec<DirectionKey> {
    let mut keys = Vec::new();
    let mut pos = NUMERIC_KEY_START;
    for &numeric_key in numeric_keys {
        let to = numeric_key_to_position(numeric_key);
        keys.append(&mut positions_to_keys(pos, to, NUMERIC_KEY_GAP));
        pos = to;
    }
    keys
}

fn direction_keys_to_direction_keys(direction_keys:&Vec<DirectionKey>) -> Vec<DirectionKey> {
    let mut keys = Vec::new();
    let mut pos = DIRECTION_KEY_START;
    for &direction_key in direction_keys {
        let to = direction_key_to_position(direction_key);
        keys.append(&mut positions_to_keys(pos, to, DIRECTION_KEY_GAP));
        pos = to;
    }
    keys
}

fn direction_keys_for_code(code:&str) -> Vec<DirectionKey> {
    let numeric_keys = code.chars().collect::<Vec<char>>();
    let direction_keys = numeric_keys_to_direction_keys(&numeric_keys);
    let direction_keys2 = direction_keys_to_direction_keys(&direction_keys);
    let direction_keys3 = direction_keys_to_direction_keys(&direction_keys2);
    direction_keys3
}

fn calculate_complexity(code:&str) -> u32 {

    let direction_keys = direction_keys_for_code(code);
    let length = direction_keys.len() as u32;

    let numeric:u32 = code.split('A').next().unwrap().parse().unwrap();

    length * numeric
}

#[test]
fn test() {
    let numeric_keys = "029A".chars().collect::<Vec<char>>();
    let direction_keys = numeric_keys_to_direction_keys(&numeric_keys);
    assert_eq!(String::from_iter(&direction_keys), "<A^A>^^AvvvA");

    let direction_keys2 = direction_keys_to_direction_keys(&direction_keys);
    //assert_eq!(String::from_iter(direction_keys2), "<v<A>>^A<A>AvA<^AA>A<vAAA>^A");
    assert_eq!(String::from_iter(direction_keys2), "v<<A>>^A<A>AvA<^AA>Av<AAA>^A");

    let direction_keys2_alt = "v<<A>>^A<A>AvA<^AA>A<vAAA>^A".chars().collect::<Vec<char>>();
    let direction_keys3 = direction_keys_to_direction_keys(&direction_keys2_alt);
    //assert_eq!(String::from_iter(direction_keys3), "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A");
    assert_eq!(String::from_iter(direction_keys3), "v<A<AA>>^AvAA<^A>Av<<A>>^AvA^Av<A>^Av<<A>^A>AAvA^Av<<A>A>^AAAvA<^A>A");

    println!("{}", String::from_iter(direction_keys_for_code("029A")));
    println!("{}", String::from_iter(direction_keys_for_code("980A")));
    println!("{}", String::from_iter(direction_keys_for_code("179A")));
    println!("{}", String::from_iter(direction_keys_for_code("456A")));
    println!("{}", String::from_iter(direction_keys_for_code("379A")));

    assert_eq!(calculate_complexity("029A"), 68 *  29);
    assert_eq!(calculate_complexity("980A"), 60 * 980);
    assert_eq!(calculate_complexity("179A"), 68 * 179);
    assert_eq!(calculate_complexity("456A"), 64 * 456);
    assert_eq!(calculate_complexity("379A"), 64 * 379);
}
