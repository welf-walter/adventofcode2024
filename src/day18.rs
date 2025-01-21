use crate::maps::FromChar;
use crate::maps::Position;
use crate::maps::PixelMap;
type Positions = Vec<Position>;

fn parse_input(lines:Vec<&str>) -> Positions {
    lines.iter().map(|&line| 
        { 
            let mut split = line.split(',');
            (split.next().unwrap().parse().unwrap(),
             split.next().unwrap().parse().unwrap())
        }
    ).collect() 
}

impl FromChar for bool {
    fn from_char(c:char) -> Self {
        match c {
            '.' => false,
            '#' => true,
            _ => unreachable!()
        }
    }
}

#[cfg(test)]
fn input1() -> &'static str {
"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"    
}


#[test]
fn test_example1() {
    let input = input1();
    let lines = input.split('\n').collect();
    let positions = parse_input(lines);
    assert_eq!(positions[3], (3,0));

    let map = PixelMap::<bool>::new(7,7,false);
 //   drop_n(&mut map, 12);
}