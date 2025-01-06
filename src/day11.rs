use std::collections::HashMap;

type Stone = u64;

type Stones = Vec<Stone>;

fn split_even_numbered_stone(stone:Stone) -> Option<(Stone, Stone)> {
    let stone_string = stone.to_string();
    let len = stone_string.len();
    if len % 2 == 0 {
        let stone1_str = &stone_string[.. len / 2];
        let stone2_str = &stone_string[   len / 2 ..];
        Some((stone1_str.parse::<Stone>().unwrap(),
              stone2_str.parse::<Stone>().unwrap()))
    } else {
        None
    }
}

#[test]
fn test_split() {
    assert_eq!(split_even_numbered_stone(1), None);
    assert_eq!(split_even_numbered_stone(12), Some((1,2)));
    assert_eq!(split_even_numbered_stone(123), None);
    assert_eq!(split_even_numbered_stone(1234), Some((12,34)));
}

fn change_stones(before:Stones) -> Stones {
    let mut after = Stones::new();
    for stone in before {
        if stone == 0 {
            after.push(1);
        } else if let Some((new1, new2)) = split_even_numbered_stone(stone) {
            after.push(new1);
            after.push(new2);
        } else {
            after.push(stone * 2024);
        }
    }
    after
}

struct Cache {
    cache:HashMap<(Stone,usize), usize>
}

impl Cache {
    fn new() -> Self {
        Self {
            cache:HashMap::new()
        }
    }

    fn number_of_stones_after_blinking_n_times(&mut self, initial_stone:Stone, n:usize) -> usize {
        if n == 0 {
            return 1;
        }
        let cached = self.cache.get(&(initial_stone, n));
        if let Some(count) = cached {
            return *count;
        }
        if initial_stone == 0 {
            let new_count = self.number_of_stones_after_blinking_n_times(1, n-1);
            self.cache.insert((1, n-1), new_count);
            return new_count;
        }
        if let Some((new1, new2)) = split_even_numbered_stone(initial_stone) {
            let new_count1 = self.number_of_stones_after_blinking_n_times(new1, n-1);
            self.cache.insert((new1, n-1), new_count1);
            let new_count2 = self.number_of_stones_after_blinking_n_times(new2, n-1);
            self.cache.insert((new2, n-1), new_count2);
            return new_count1 + new_count2;
        }
        let new = initial_stone*2024;
        let new_count = self.number_of_stones_after_blinking_n_times(new, n-1);
        self.cache.insert((new, n-1), new_count);
        return new_count;
    }
}

#[test]
fn test_change_stones() {
    let input1  = vec![0,    1,   10,   99,     999];
    let expect1 = vec![1, 2024, 1, 0, 9, 9, 2021976];
    assert_eq!(change_stones(input1), expect1);
    let mut cache=Cache::new();
    assert_eq!(cache.number_of_stones_after_blinking_n_times(0,1), 1);
    assert_eq!(cache.number_of_stones_after_blinking_n_times(1,1), 1);
    assert_eq!(cache.number_of_stones_after_blinking_n_times(10,1), 2);
    assert_eq!(cache.number_of_stones_after_blinking_n_times(99,1), 2);
    assert_eq!(cache.number_of_stones_after_blinking_n_times(999,1), 1);

    let initial = vec![125, 17];
    let blink1 = change_stones(initial);
    assert_eq!(blink1, vec![253000, 1, 7]);
    let blink2 = change_stones(blink1);
    assert_eq!(blink2, vec![253, 0, 2024, 14168]);
    let blink3 = change_stones(blink2);
    assert_eq!(blink3, vec![512072, 1, 20, 24, 28676032]);
    let blink4 = change_stones(blink3);
    assert_eq!(blink4, vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032]);
    let blink5 = change_stones(blink4);
    assert_eq!(blink5, vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32]);
    let blink6 = change_stones(blink5);
    assert_eq!(blink6, vec![2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6, 0, 3, 2]);
    assert_eq!(blink6.len(), 22);
    let mut blinkn = blink6;
    for _ in 6..25 {
        blinkn = change_stones(blinkn);
    }
    assert_eq!(blinkn.len(), 55312);
}

//////////////////////////////////////////
/// Puzzle
//////////////////////////////////////////

pub fn puzzle() {
    let lines = crate::helper::read_file("input/day11.txt");
    let input = lines[0].split(' ').map(|s| s.parse::<Stone>().unwrap()).collect();
    let mut stones = input;
    for _ in 0..25 {
        stones = change_stones(stones);
    }
    println!("Day 11, Part 1: Number of stones after blinking 25 times is {}", stones.len());
    for _ in 25..75 {
        stones = change_stones(stones);
    }
    println!("Day 11, Part 2: Number of stones after blinking 75 times is {}", stones.len());
}