pub type Position = (/* x: */usize,/* y: */usize);

//////////////////////////////////////////
/// Direction
//////////////////////////////////////////

#[derive(Eq, Hash, PartialEq, Debug,Copy,Clone)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
    UpRight,
    DownRight,
    DownLeft,
    UpLeft,
}

impl FromChar for Direction {
    fn from_char(c:char) -> Self {
        match c {
            '^' => Direction::Up,
            '<' => Direction::Left,
            '>' => Direction::Right,
            'v' => Direction::Down,
            _   => panic!("Unexpected direction character {}", c)
        }
     }
}

impl Direction {
    pub fn turn_right(&self) -> Direction {
        match self {
            Up    => Right,
            Right => Down,
            Down  => Left,
            Left  => Up,
            UpRight   => DownRight,
            DownRight => DownLeft,
            DownLeft  => UpLeft,
            UpLeft    => UpRight
        }
    }

    pub fn turn_left(&self) -> Direction {
        match self {
            Up    => Left,
            Left  => Down,
            Down  => Right,
            Right => Up,
            DownRight => UpRight,
            DownLeft  => DownRight,
            UpLeft    => DownLeft,
            UpRight   => UpLeft
        }
    }
    pub fn four_directions() -> [Direction;4] {
        [Right, Down, Left, Up]
    }
}

use crate::maps::Direction::*;

#[test]
fn test_direction() {
    assert_eq!(Left.turn_right().turn_right().turn_right(), Left.turn_left());
}

//////////////////////////////////////////
/// Area
//////////////////////////////////////////

// some bounded area to move in
#[derive(PartialEq, Debug, Clone)]
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

    pub fn position_add(&self, pos:Position, delta_x:i32, delta_y:i32) -> Option<Position> {
        let new_x:i32 = pos.0 as i32 + delta_x;
        let new_y:i32 = pos.1 as i32 + delta_y;
        if new_x < 0 || new_x >= self.width as i32 ||
           new_y < 0 || new_y >= self.height as i32 {
            return None;
           }
        Some((new_x as usize, new_y as usize))
    }

    // return None if out of area
    pub fn step(&self, pos:Position, direction:Direction) -> Option<Position> {
        let w = self.width-1;
        let h = self.height-1;
        match direction {
            Up    => { if pos.1 > 0 { return Some((pos.0  ,pos.1-1)); } else { return None; }},
            Right => { if pos.0 < w { return Some((pos.0+1,pos.1  )); } else { return None; }},
            Down  => { if pos.1 < h { return Some((pos.0  ,pos.1+1)); } else { return None; }},
            Left  => { if pos.0 > 0 { return Some((pos.0-1,pos.1  )); } else { return None; }},
            UpRight   => { if (pos.0 < w) & (pos.1 > 0) { return Some((pos.0+1,pos.1-1)); } else { return None; }},
            DownRight => { if (pos.0 < h) & (pos.1 < w) { return Some((pos.0+1,pos.1+1)); } else { return None; }},
            DownLeft  => { if (pos.0 > 0) & (pos.1 < w) { return Some((pos.0-1,pos.1+1)); } else { return None; }},
            UpLeft    => { if (pos.0 > 0) & (pos.1 > 0) { return Some((pos.0-1,pos.1-1)); } else { return None; }}
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
    assert_eq!(area.step((2,2), Left), Some((1,2)));
    assert_eq!(area.position_add((2,2), -1, -2), Some((1,0)));
    assert_eq!(area.position_add((2,2),  1, -2), None);
    assert_eq!(area.all_positions().collect::<Vec<Position>>(), vec![
        (0,0),(1,0),(2,0),
        (0,1),(1,1),(2,1),
        (0,2),(1,2),(2,2)
    ]);
}

//////////////////////////////////////////
/// EnumMap
//////////////////////////////////////////

pub trait FromChar {
    fn from_char(c:char) -> Self;
}

pub trait ToChar {
    fn to_char(self) -> char;
}

impl ToChar for char {
    fn to_char(self) -> char {
        self
    }
}

#[derive(Clone)]
pub struct PixelMap<E:FromChar> {
    pub area:Area,
    pub pixels:Vec<Vec<E>>
}

impl<E:FromChar+Copy+PartialEq> PixelMap<E> {
    #[cfg(test)]
    pub fn width(&self) -> usize { self.area.width }
    #[cfg(test)]
    pub fn height(&self) -> usize { self.area.height }

    pub fn at(&self, position:Position) -> E {
        self.pixels[position.1][position.0]
    }

    pub fn set_at(&mut self, position:Position, value:E) {
        self.pixels[position.1][position.0] = value;
    }

    pub fn find_first(&self, value:E) -> Option<Position> {
        for pos in self.area.all_positions() {
            if self.at(pos) == value {
                return Some(pos);
            }
        }
        None
    }

    pub fn new(width:usize, height:usize, init_value:E) -> Self {
        let mut pixels = Vec::new();

        for _y in 0..height {
            let mut line = Vec::new();
            for _x in 0..width {
                line.push(init_value);
            }
            pixels.push(line);
        }
        Self{area:Area{width,height}, pixels}
    }

    pub fn from_strings<'a>(lines:impl Iterator<Item=&'a str>) -> Self {
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
        Self{area:Area{width,height}, pixels}
    }

}

impl<E:FromChar+ToChar+Clone+Copy> PixelMap<E> {
    pub fn println(&self) {
        for y in 0..self.area.height {
            let line = &self.pixels[y];
            for x in 0..self.area.width {
                print!("{}", line[x].to_char());
            }
            println!("");
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum TestEnum {
    A,
    B,
    C
}

impl FromChar for char {
    fn from_char(c:char) -> Self { c }
}

impl FromChar for TestEnum {
    fn from_char(c:char) -> Self {
        match c {
            'A' => TestEnum::A,
            'B' => TestEnum::B,
            'C' => TestEnum::C,
            _ => panic!("Unexpected character {} for TestEnum", c)
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
    assert_eq!(pixel_map.area.all_positions().map( |pos| pixel_map.at(pos) ).collect::<Vec<_>>(), vec![
        TestEnum::A, TestEnum::B, TestEnum::C,
        TestEnum::B, TestEnum::C, TestEnum::A
    ]);
}