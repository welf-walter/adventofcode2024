pub type Position = (/* x: */usize,/* y: */usize);

//////////////////////////////////////////
/// Direction
//////////////////////////////////////////

#[derive(Eq, Hash, PartialEq, Debug,Copy,Clone)]
pub enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT
}

impl Direction {
    pub fn turn_right(&self) -> Direction {
        match self {
            UP    => RIGHT,
            RIGHT => DOWN,
            DOWN  => LEFT,
            LEFT  => UP
        }
    }
    #[cfg(test)]
    pub fn turn_left(&self) -> Direction {
        match self {
            UP    => LEFT,
            LEFT  => DOWN,
            DOWN  => RIGHT,
            RIGHT => UP
        }
    }
}

use crate::maps::Direction::*;

#[test]
fn test_direction() {
    assert_eq!(LEFT.turn_right().turn_right().turn_right(), LEFT.turn_left());
}

//////////////////////////////////////////
/// Area
//////////////////////////////////////////

// some bounded area to move in
#[derive(PartialEq, Debug)]
pub struct Area {
    pub width:usize,
    pub height:usize
}

pub struct AreaIterator<'a> {
    area:&'a Area,
    // next x
    x:usize,
    // next y
    y:usize,
    done:bool
}

impl Iterator for AreaIterator<'_> {
    type Item = Position;
    fn next(&mut self) -> Option<Position> {
        if self.done { return None; }
        let pos=(self.x, self.y);
        if self.x < self.area.width-1  { self.x +=1; return Some(pos)};
        self.x = 0;
        if self.y < self.area.height-1 { self.y += 1; return Some(pos)};
        self.y = 0;
        self.done = true;
        return Some(pos);
    }
}

impl Area {
    pub fn contains_signed(&self, x:i32, y:i32) -> bool {
        x >= 0 && (x as usize) < self.width && y >= 0 && (y as usize) < self.height
    }
    // return None if out of area
    pub fn step(&self, pos:Position, direction:Direction) -> Option<Position> {
        match direction {
            UP    => { if pos.1 > 0             { return Some((pos.0  ,pos.1-1)); } else { return None; }},
            RIGHT => { if pos.0 < self.width-1  { return Some((pos.0+1,pos.1  )); } else { return None; }},
            DOWN  => { if pos.1 < self.height-1 { return Some((pos.0  ,pos.1+1)); } else { return None; }},
            LEFT  => { if pos.0 > 0             { return Some((pos.0-1,pos.1  )); } else { return None; }}
        }
    }

    pub fn all_positions(&self) -> AreaIterator {
        AreaIterator{area:&self, x:0,y:0, done:false}
    }
}

#[test]
fn test_area() {
    let area = Area{width:3,height:3};
    assert_eq!(area.contains_signed(2,2), true);
    assert_eq!(area.contains_signed(2,3), false);
    assert_eq!(area.contains_signed(2,-1), false);
    assert_eq!(area.step((2,2), LEFT), Some((1,2)));
    assert_eq!(area.all_positions().collect::<Vec<Position>>(), vec![
        (0,0),(1,0),(2,0),
        (0,1),(1,1),(2,1),
        (0,2),(1,2),(2,2)
    ]);
}

//////////////////////////////////////////
/// EnumMap
//////////////////////////////////////////

pub trait CharBijection {
    fn from_char(c:char) -> Self;
    fn to_char(&self) -> char;
}

pub struct PixelMap<E:CharBijection> {
    pub area:Area,
    pixels:Vec<Vec<E>>
}

impl<E:CharBijection+Copy> PixelMap<E> {
    pub fn width(&self) -> usize { self.area.width }
    pub fn height(&self) -> usize { self.area.height }

    fn at(&self, position:Position) -> E {
        self.pixels[position.1][position.0]
    }

    fn from_strings<'a>(lines:impl Iterator<Item=&'a str>) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut pixels = Vec::new();
        for line in lines {
            width = 0;
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(E::from_char(c));
                width = width + 1;
            }
            pixels.push(row);
            height = height + 1;
        }
        Self{area:Area{width:width,height:height}, pixels:pixels}
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum TestEnum {
    A,
    B,
    C
}

impl CharBijection for TestEnum {
    fn from_char(c:char) -> Self {
        match c {
            'A' => TestEnum::A,
            'B' => TestEnum::B,
            'C' => TestEnum::C,
            _ => panic!("Unexpected character {} for TestEnum", c)
        }
    }
    fn to_char(&self) -> char {
        match self {
            TestEnum::A => 'A',
            TestEnum::B => 'B',
            TestEnum::C => 'C'
        }
    }
}

#[test]
fn test_from_strings() {
    type TestMap = PixelMap<TestEnum>;
    let input =
"ABC
BCA";
    let pixel_map = TestMap::from_strings(input.split('\n'));
    assert_eq!(pixel_map.area, Area{width:3, height:2});
    assert_eq!(pixel_map.width(), 3);
    assert_eq!(pixel_map.height(), 2);
    assert_eq!(pixel_map.pixels, vec![
        vec![TestEnum::A, TestEnum::B, TestEnum::C],
        vec![TestEnum::B, TestEnum::C, TestEnum::A]
    ]);
    assert_eq!(pixel_map.at((1,1)), TestEnum::C);
    assert_eq!(pixel_map.pixels.iter().map(|line| line.iter().map( |e| e.to_char()).collect::<Vec<_>>()).collect::<Vec<_>>(), vec![
        vec!['A', 'B', 'C'],
        vec!['B', 'C', 'A']
    ]);
}