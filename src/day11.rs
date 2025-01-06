type Stone = u32;

type Stones = Vec<Stone>;

fn split_even_numbered_stone(stone:Stone) -> Option<(Stone, Stone)> {
    let stone_string = stone.to_string();
    let len = stone_string.len();
    if len % 2 == 0 {
        let stone1_str = &stone_string[.. len / 2];
        let stone2_str = &stone_string[   len / 2 ..];
        Some((stone1_str.parse::<u32>().unwrap(),
              stone2_str.parse::<u32>().unwrap()))
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

#[test]
fn test_change_stones() {
    let input1  = vec![0,    1,   10,   99,     999];
    let expect1 = vec![1, 2024, 1, 0, 9, 9, 2021976];
    assert_eq!(change_stones(input1), expect1);
}