type Position = (/* x: */i32,/* y: */i32);

#[derive(Debug, PartialEq, Copy, Clone)]
struct Antenna {
    frequency:char,
    position:Position
}

struct Map {
    width:usize,
    height:usize,
    antennas:Vec<Antenna>
}

fn parse_map(lines:&Vec<String>) -> Map {
    let mut antennas:Vec<Antenna> = Vec::new();
    let width = lines[0].len();
    let height = lines.len();
    for y in 0..height {
        let mut x = 0;
        for c in lines[y].chars() {
            if c != '.' {
                antennas.push(Antenna{frequency:c, position:(x as i32,y as i32)});
            }
            x += 1;
        }
    }
    Map { width, height, antennas }
}

#[cfg(test)]
fn input1() -> Vec<String> {
"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............".split('\n').map(|line| line.to_string()).collect()
}

#[test]
fn test_parse() {
    let map = parse_map(&input1());
    assert_eq!(map.width, 12);
    assert_eq!(map.height, 12);
    assert_eq!(map.antennas.len(), 7);
    assert_eq!(map.antennas[4], Antenna{frequency:'A', position:(6,5)});
}