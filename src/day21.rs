
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

#[cfg(test)]
fn vecvec_to_strvec(vecvec:Vec<Vec<DirectionKey>>) -> Vec<String> {
    vecvec.iter().map(|vec| String::from_iter(vec.iter())).collect()
}

#[cfg(test)]
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


fn best_keys_for_direction_keys_n(direction_keys:&Vec<DirectionKey>, n:u32) -> Vec<DirectionKey> {
    let mut keys = Vec::new();
    let mut pos = DIRECTION_KEY_START;
    for &direction_key in direction_keys {
        let to = direction_key_to_position(direction_key);
        let all_possible_keys = positions_to_all_possible_keys(pos, to, DIRECTION_KEY_GAP);

        if n == 1 {
            // when at end of recursion, all sequences are equally good
            let mut any_keys = all_possible_keys[0].clone();
            keys.append(&mut any_keys);
        } else {
            let all_keys:Vec<Vec<DirectionKey>> =
            all_possible_keys.iter().map(|keys|best_keys_for_direction_keys_n(keys, n-1)).collect();
            let min = all_keys.iter().map(|x|x.len()).min().unwrap();
            let best_index = all_keys.iter().position(|x| x.len() == min).unwrap();
            let mut best_keys = all_keys[best_index].clone();

            keys.append(&mut best_keys);
        }

        pos = to;
    }

    keys
}

fn best_keys_for_numeric_keys_n(numeric_keys:&Vec<NumericKey>, n:u32) -> Vec<DirectionKey> {
    let mut keys = Vec::new();
    let mut pos = NUMERIC_KEY_START;
    for &numeric_key in numeric_keys {
        let to = numeric_key_to_position(numeric_key);
        let all_possible_keys = positions_to_all_possible_keys(pos, to, NUMERIC_KEY_GAP);

        if n == 0 {
            // when no recursion, all sequences are equally good
            let mut any_keys = all_possible_keys[0].clone();
            keys.append(&mut any_keys);
        } else {
            let all_keys:Vec<Vec<DirectionKey>> = all_possible_keys.iter().map(|keys|best_keys_for_direction_keys_n(keys, n)).collect();
            let min = all_keys.iter().map(|x|x.len()).min().unwrap();
            let best_index = all_keys.iter().position(|x| x.len() == min).unwrap();
            let mut best_keys = all_keys[best_index].clone();

            keys.append(&mut best_keys);
        }

        pos = to;
    }
    keys
}

fn calculate_complexity(code:&str, keys:&Vec<DirectionKey>) -> u32 {
    let code_int:u32 = code[0..3].parse().unwrap();
    code_int * keys.len() as u32
}

#[test]
fn test() {
    let code1 = "029A";
    let numeric_keys1 = code1.chars().collect::<Vec<char>>();

    assert_eq!(vec_to_str(&best_keys_for_numeric_keys_n(&numeric_keys1, 0)), "<A^A>^^AvvvA");
    assert_eq!(vec_to_str(&best_keys_for_numeric_keys_n(&numeric_keys1, 1)), "v<<A>>^A<A>AvA<^AA>Av<AAA>^A");
    assert_eq!(vec_to_str(&best_keys_for_numeric_keys_n(&numeric_keys1, 2)), "v<A<AA>>^AvAA<^A>Av<<A>>^AvA^Av<A>^Av<<A>^A>AAvA^Av<A<A>>^AAAvA<^A>A");

    let result1 = best_keys_for_numeric_keys_n(&numeric_keys1, 2);

    let code2 = "980A";
    let result2 = best_keys_for_numeric_keys_n(&code2.chars().collect(), 2);
                                          //<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A
    assert_eq!(vec_to_str(&result2), "v<<A>>^AAAvA^Av<A<AA>>^AvAA<^A>Av<A<A>>^AAAvA<^A>Av<A>^A<A>A");


    let code3 = "179A";
    let result3 = best_keys_for_numeric_keys_n(&code3.chars().collect(), 2);
                                          //<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
    assert_eq!(vec_to_str(&result3), "v<<A>>^Av<A<A>>^AAvAA<^A>Av<<A>>^AAvA^Av<A>^AA<A>Av<A<A>>^AAAvA<^A>A");

    let code4 = "456A";
    let result4 = best_keys_for_numeric_keys_n(&code4.chars().collect(), 2);
    println!("{}", vec_to_str(&result4));
                                          //<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A
    assert_eq!(vec_to_str(&result4), "v<<A>>^AAv<A<A>>^AAvAA<^A>Av<A>^A<A>Av<A>^A<A>Av<A<A>>^AAvA<^A>A");

    let code5 = "379A";
    let result5 = best_keys_for_numeric_keys_n(&code5.chars().collect(), 2);
    //                                      <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
    assert_eq!(vec_to_str(&result4), "v<<A>>^AAv<A<A>>^AAvAA<^A>Av<A>^A<A>Av<A>^A<A>Av<A<A>>^AAvA<^A>A");

    assert_eq!(calculate_complexity(code1, &result1), 68 * 29);
    assert_eq!(calculate_complexity(code2, &result2), 60 * 980);
    assert_eq!(calculate_complexity(code3, &result3), 68 * 179);
    assert_eq!(calculate_complexity(code4, &result4), 64 * 456);
    assert_eq!(calculate_complexity(code5, &result5), 64 * 379);

}


//////////////////////////////////////////
/// Puzzle
//////////////////////////////////////////

use std::time::Instant;

pub fn puzzle() {
    let lines = crate::helper::read_file("input/day21.txt");

    let results = lines.iter().
        map(|code| (code, best_keys_for_numeric_keys_n(&code.chars().collect(),2)));
    let complexity:u32 = results.map(|(code, result)| calculate_complexity(code, &result)).sum();

    println!("Day 21, Part 1: Sum of 2 level complexities for {} codes is {}", lines.len(), complexity);

    for n in 3..25 {
        let start = Instant::now();

        let results2 = lines.iter().
            map(|code| (code, best_keys_for_numeric_keys_n(&code.chars().collect(), n)));
        let complexity2:u32 = results2.map(|(code, result)| calculate_complexity(code, &result)).sum();

        println!("Day 21, Part 2: Sum of {} level complexities for {} codes is {} (calculated in {} ms)", n, lines.len(), complexity2, start.elapsed().as_millis());
    }

}
