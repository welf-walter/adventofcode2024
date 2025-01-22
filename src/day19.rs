type Design = String;
type Designs = Vec<String>;
type Towel = String;
type Towels = Vec<String>;

fn is_design_possible(design:Design, towels:Towels) -> bool {
    false
}

fn read_input(input:Vec<String>) -> (Towels, Designs) {
    let mut iter = input.iter();
    let towels_line = iter.next().unwrap();
    let towels = towels_line.split(", ").map(|str| str.to_string()).collect::<Vec<_>>();

    assert_eq!(iter.next().unwrap(), "");

    let mut designs = Designs::new();
    for line in iter {
        designs.push(line.clone());
    }

    (towels, designs)
}

#[cfg(test)]
fn input1() -> Vec<String> {
"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb".split('\n').map(|str| str.to_string()).collect::<Vec<_>>()
}

#[test]
fn test_example1() {
    let (towels, designs) = read_input(input1());
    assert_eq!(towels.len(), 8);
    assert_eq!(&towels[4], "bwu");
    assert_eq!(designs.len(), 8);
    assert_eq!(&designs[4], "ubwu");
}