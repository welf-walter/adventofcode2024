struct Puzzle {
    width:usize,
    height:usize,
    letters:Vec<Vec<char>>
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
        let height = rows.len();
        let width = rows[0].len();
        for row in &rows {
            assert_eq!(row.len(),width);
        }
        Puzzle { width, height, letters: rows }
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