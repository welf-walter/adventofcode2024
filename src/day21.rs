
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

fn positions_to_all_possible_keys(from:Position, to:Position, invalid_position:Position) -> Vec<Vec<DirectionKey>> {
    let mut all_keys = Vec::new();

    if from == to {
        return vec![vec!['A']];
    }

    if from.0 < to.0 && right(from) != invalid_position { // Right
        let all_keys_to_now = positions_to_all_possible_keys(right(from), to, invalid_position);
        for mut keys in all_keys_to_now.into_iter() {
            keys.insert(0, '>');
            all_keys.push(keys);
        }
    }

    if from.1 < to.1 && down(from) != invalid_position { // Down
        let all_keys_to_now = positions_to_all_possible_keys(down(from), to, invalid_position);
        for mut keys in all_keys_to_now.into_iter() {
            keys.insert(0, 'v');
            all_keys.push(keys);
        }
    }

    if from.0 > to.0 && left(from) != invalid_position { // Left
        let all_keys_to_now = positions_to_all_possible_keys(left(from), to, invalid_position);
        for mut keys in all_keys_to_now.into_iter() {
            keys.insert(0, '<');
            all_keys.push(keys);
        }
    }

    if from.1 > to.1 && up(from) != invalid_position { // Up
        let all_keys_to_now = positions_to_all_possible_keys(up(from), to, invalid_position);
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

fn vec_to_str(vec:&Vec<DirectionKey>) -> String {
    String::from_iter(vec.iter())
}

#[test]
fn test_positions_to_all_possible_keys() {
    assert_eq!(vecvec_to_strvec(positions_to_all_possible_keys((2,2),(2,2),(0,0))),vec!["A"]);
    assert_eq!(vecvec_to_strvec(positions_to_all_possible_keys((2,2),(3,2),(0,0))),vec![">A"]);
    assert_eq!(vecvec_to_strvec(positions_to_all_possible_keys((2,2),(3,3),(0,0))),vec![">vA","v>A"]);
    assert_eq!(vecvec_to_strvec(positions_to_all_possible_keys((2,2),(3,3),(2,3))),vec![">vA"]);
}

fn best_keys_for_direction_keys2(direction_keys:&Vec<DirectionKey>) -> Vec<DirectionKey> {
    let mut keys = Vec::new();
    let mut pos = DIRECTION_KEY_START;
    for &numeric_key in direction_keys {
        let to = direction_key_to_position(numeric_key);
        let all_possible_keys = positions_to_all_possible_keys(pos, to, DIRECTION_KEY_GAP);
        // todo: find best of these
        let mut any_possible_key = all_possible_keys[0].clone();
        keys.append(&mut any_possible_key);
        pos = to;
    }
    keys
}

struct Result2 {
    keys2:Vec<DirectionKey>,
    keys3:Vec<DirectionKey>
}

fn best_keys_for_direction_keys1(direction_keys:&Vec<DirectionKey>) -> Result2 {
    let mut keys2 = Vec::new();
    let mut keys3 = Vec::new();
    let mut pos = DIRECTION_KEY_START;
    for &numeric_key in direction_keys {
        let to = direction_key_to_position(numeric_key);
        let all_possible_keys = positions_to_all_possible_keys(pos, to, DIRECTION_KEY_GAP);

        let all_keys3:Vec<Vec<DirectionKey>> = all_possible_keys.iter().map(best_keys_for_direction_keys2).collect();
        let min3 = all_keys3.iter().map(|x|x.len()).min().unwrap();
        let best_index = all_keys3.iter().position(|x| x.len() == min3).unwrap();
        let mut best_keys3 = all_keys3[best_index].clone();
        let mut best_keys2 = all_possible_keys[best_index].clone();

        keys2.append(&mut best_keys2);
        keys3.append(&mut best_keys3);

        pos = to;
    }
    Result2 {keys2, keys3}
}

struct Result {
    keys1:Vec<DirectionKey>,
    keys2:Vec<DirectionKey>,
    keys3:Vec<DirectionKey>
}

fn best_keys_for_numeric_keys(numeric_keys:&Vec<NumericKey>) -> Result {
    let mut keys1 = Vec::new();
    let mut keys2 = Vec::new();
    let mut keys3 = Vec::new();
    let mut pos = NUMERIC_KEY_START;
    for &numeric_key in numeric_keys {
        let to = numeric_key_to_position(numeric_key);
        let all_possible_keys = positions_to_all_possible_keys(pos, to, NUMERIC_KEY_GAP);
        let all_keys2:Vec<Result2> = all_possible_keys.iter().map(best_keys_for_direction_keys1).collect();
        let min3 = all_keys2.iter().map(|x|x.keys3.len()).min().unwrap();
        let best_index = all_keys2.iter().position(|x| x.keys3.len() == min3).unwrap();
        let mut best_keys3 = all_keys2[best_index].keys3.clone();
        let mut best_keys2 = all_keys2[best_index].keys2.clone();
        let mut best_keys = all_possible_keys[best_index].clone();

        keys1.append(&mut best_keys);
        keys2.append(&mut best_keys2);
        keys3.append(&mut best_keys3);
        pos = to;
    }
    Result{keys1, keys2, keys3}
}

fn calculate_complexity(code:&str, keys3:&Vec<DirectionKey>) -> u32 {
    let code_int:u32 = code[0..3].parse().unwrap();
    code_int * keys3.len() as u32
}

#[test]
fn test() {
    let code1 = "029A";
    let numeric_keys1 = code1.chars().collect::<Vec<char>>();
    let result1 = best_keys_for_numeric_keys(&numeric_keys1);

    assert_eq!(vec_to_str(&result1.keys1), "<A^A>^^AvvvA");
//    assert_eq!(result.keys2, "v<<A>>^A<A>AvA<^AA>A<vAAA>^A".chars().collect::<Vec<char>>());
    assert_eq!(vec_to_str(&result1.keys2), "v<<A>>^A<A>AvA<^AA>Av<AAA>^A");
//    assert_eq!(result.keys3, "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".chars().collect::<Vec<char>>());
    assert_eq!(vec_to_str(&result1.keys3), "v<A<AA>>^AvAA<^A>Av<<A>>^AvA^Av<A>^Av<<A>^A>AAvA^Av<A<A>>^AAAvA<^A>A");

    let code2 = "980A";
    let result2 = best_keys_for_numeric_keys(&code2.chars().collect());
                                          //<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A
    assert_eq!(vec_to_str(&result2.keys3), "v<<A>>^AAAvA^Av<A<AA>>^AvAA<^A>Av<A<A>>^AAAvA<^A>Av<A>^A<A>A");


    let code3 = "179A";
    let result3 = best_keys_for_numeric_keys(&code3.chars().collect());
                                          //<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
    assert_eq!(vec_to_str(&result3.keys3), "v<<A>>^Av<A<A>>^AAvAA<^A>Av<<A>>^AAvA^Av<A>^AA<A>Av<A<A>>^AAAvA<^A>A");

    let code4 = "456A";
    let result4 = best_keys_for_numeric_keys(&code4.chars().collect());
    println!("{}", vec_to_str(&result4.keys3));
                                          //<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A
    assert_eq!(vec_to_str(&result4.keys3), "v<<A>>^AAv<A<A>>^AAvAA<^A>Av<A>^A<A>Av<A>^A<A>Av<A<A>>^AAvA<^A>A");

    let code5 = "379A";
    let result5 = best_keys_for_numeric_keys(&code5.chars().collect());
    //                                      <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
    assert_eq!(vec_to_str(&result4.keys3), "v<<A>>^AAv<A<A>>^AAvAA<^A>Av<A>^A<A>Av<A>^A<A>Av<A<A>>^AAvA<^A>A");

    assert_eq!(calculate_complexity(code1, &result1.keys3), 68 * 29);
    assert_eq!(calculate_complexity(code2, &result2.keys3), 60 * 980);
    assert_eq!(calculate_complexity(code3, &result3.keys3), 68 * 179);
    assert_eq!(calculate_complexity(code4, &result4.keys3), 64 * 456);
    assert_eq!(calculate_complexity(code5, &result5.keys3), 64 * 379);

}