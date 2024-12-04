struct Puzzle {
    width:u32,
    height:u32,
    letters:Vec<Vec<char>>
}

type Position = (/* x: */ i32,/* y: */ i32);
type Direction = (/* deltax: */ i32,/* deltay: */ i32);

fn change_position(current:Position, direction:Direction) -> Position {
    (current.0 + direction.0, current.1 + direction.1)
}

fn all_directions() -> [Direction;8] {
    [( 1, 0),( 1, 1),( 0, 1),(-1, 1),
     (-1, 0),(-1,-1),( 0,-1),( 1,-1)]
}

impl Puzzle {
    fn create<'a>(lines:impl Iterator<Item=&'a str>) -> Puzzle {
        let mut rows = Vec::new();
        for line in lines {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c);
            }
            rows.push(row);
        }
        let height = rows.len() as u32;
        let width = rows[0].len() as u32;
        for row in &rows {
            assert_eq!(row.len() as u32,width);
        }
        Puzzle { width, height, letters: rows }
    }

    fn letter_at(&self, position:Position) -> char {
        self.letters[position.1 as usize][position.0 as usize]
    }

    fn is_valid_position(&self, position:Position) -> bool {
        position.0 >= 0 && position.0 < self.width as i32 && 
        position.1 >= 0 && position.1 < self.height as i32
    }

    fn matches(&self, text:&str, position:Position, direction:Direction) -> bool {
        let mut pos = position;
        for c in text.chars() {
            if !self.is_valid_position(pos) { return false; }
            if c != self.letter_at(pos) { return false; }
            pos = change_position(pos, direction);
        }
        return true;
    }

    fn find(&self, text:&str) -> u32 {
        let mut match_count = 0;
        let all_directions = all_directions();
        for y in 0..self.height {
            for x in 0..self.width {
                let position = (x as i32,y as i32);
                for direction in &all_directions {
                    if self.matches(text, position, *direction) {
                        match_count += 1;
                    }
                }
            }
        }
        match_count
    }
}

#[test]
fn test_parsing() {
    let input1 =
"..X...
.SAMX.
.A..A.
XMAS.S
.X....";
    let puzzle1 = Puzzle::create(input1.split("\n"));
    assert_eq!(puzzle1.width, 6);
    assert_eq!(puzzle1.height, 5);
    assert_eq!(puzzle1.letters[1], vec!['.','S','A','M','X','.']);
}

#[test]
fn test_move() {
    let p = (2,3);
    assert_eq!(change_position(p,(1,0)),(3,3));
}

#[test]
fn test_puzzle() {
    let input1 =
"..X...
.SAMX.
.A..A.
XMAS.S
.X....";
    let puzzle1 = Puzzle::create(input1.split('\n'));
    assert_eq!(puzzle1.matches("XMAS",(0,3),(1,0)), true);
    assert_eq!(puzzle1.matches("XMAS",(1,1),(1,0)), false);
    assert_eq!(puzzle1.matches("XMAS",(4,1),(1,0)), false);
    assert_eq!(puzzle1.find("XMAS"), 4);

    let input2 = 
"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    let puzzle2 = Puzzle::create(input2.split('\n'));
    assert_eq!(puzzle2.find("XMAS"), 18);
}