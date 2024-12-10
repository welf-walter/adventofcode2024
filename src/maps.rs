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
pub struct Area {
    pub width:usize,
    pub height:usize
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
}
