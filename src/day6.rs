use std::collections::HashSet;

type Position = (/* x: */usize,/* y: */usize);

enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT
}

struct Map {
    width:usize,
    height:usize,
    obstructions:HashSet<Position>,
    start:Position // Direction is up
}

fn read_map(lines:Vec<String>) -> Map {
    let height = lines.len();
    let width = lines[0].len();
    let mut obstructions:HashSet<Position> = HashSet::new();
    let mut start:Position = (999,999);
    for y in 0..height {
        let chars = lines[y].chars();
        let mut x = 0;
        for c in chars {
            match c {
                '.' => { },
                '#' => { obstructions.insert((x,y)); },
                '^' => { start = (x,y); },
                _ => panic!("Unexpected character '{}'", c)
            }
            x += 1;
        }
    }
    Map { width, height, obstructions, start }

}

#[cfg(test)]
fn input1() -> Vec<String> {
"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...".split('\n').map(|s| s.to_string()).collect()
}

#[test]
fn test_map() 
{
    let map = read_map(input1());
    assert_eq!(map.width, 10);
    assert_eq!(map.height, 10);
    assert_eq!(map.start, (4,6));
    assert_eq!(map.obstructions.len(), 8);
    assert!(map.obstructions.contains(&(2,3)));
}