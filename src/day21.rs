
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

fn positions_to_all_possible_keys(from:Position, to:Position) -> Vec<Vec<DirectionKey>> {
    let mut all_keys = Vec::new();
    let mut current = from;

    if from == to {
        return vec![vec!['A']];
    }

    if from.0 < to.0 { // Right
        let all_keys_to_now = positions_to_all_possible_keys((from.0+1,from.1  ), to);
        for mut keys in all_keys_to_now.into_iter() {
            keys.insert(0, '>');
            all_keys.push(keys);
        }
    }

    if from.1 < to.1 { // Down
        let all_keys_to_now = positions_to_all_possible_keys((from.0  ,from.1+1), to);
        for mut keys in all_keys_to_now.into_iter() {
            keys.insert(0, 'v');
            all_keys.push(keys);
        }
    }

    if from.0 > to.0 { // Left
        let all_keys_to_now = positions_to_all_possible_keys((from.0-1,from.1  ), to);
        for mut keys in all_keys_to_now.into_iter() {
            keys.insert(0, '<');
            all_keys.push(keys);
        }
    }

    if from.1 > to.1 { // Up
        let all_keys_to_now = positions_to_all_possible_keys((from.0  ,from.1-1), to);
        for mut keys in all_keys_to_now.into_iter() {
            keys.insert(0, '^');
            all_keys.push(keys);
        }
    }

    all_keys
}

fn vecvec_to_strvec(vecvec:Vec<Vec<DirectionKey>>) -> Vec<String> {
    vecvec.iter().map(|vec| String::from_iter(vec.iter())).collect()
}

#[test]
fn test_positions_to_all_possible_keys() {
    assert_eq!(vecvec_to_strvec(positions_to_all_possible_keys((2,2),(2,2))),vec!["A"]);
    assert_eq!(vecvec_to_strvec(positions_to_all_possible_keys((2,2),(3,2))),vec![">A"]);
    assert_eq!(vecvec_to_strvec(positions_to_all_possible_keys((2,2),(3,3))),vec![">vA","v>A"]);
}


fn best_keys_for_numeric_keys(numeric_keys:&Vec<NumericKey>) -> Vec<DirectionKey> {
    let mut keys = Vec::new();
    let mut pos = NUMERIC_KEY_START;
    for &numeric_key in numeric_keys {
        let to = numeric_key_to_position(numeric_key);
        let all_possible_keys = positions_to_all_possible_keys(pos, to);
        // todo: find best of these
        let mut any_possible_key = all_possible_keys[0].clone();
        keys.append(&mut any_possible_key);
        pos = to;
    }
    keys
}

#[test]
fn test() {
    let numeric_keys = "029A".chars().collect::<Vec<char>>();
    let direction_keys = best_keys_for_numeric_keys(&numeric_keys);
    assert_eq!(direction_keys, "<A^A>^^AvvvA".chars().collect::<Vec<char>>());
}