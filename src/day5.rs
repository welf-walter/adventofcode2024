use std::cmp::Ordering;

const VERBOSE:bool = true;

type Page = u32;

// Page a must be before page b
type Rule = (/* a: */Page,/* b: */Page);

type Update = Vec<Page>;

//////////////////////////////////////////
/// Rules
//////////////////////////////////////////

struct Rules {
    rules:Vec<Rule>
}

impl Rules {
    fn is_correct_update(&self, update:&Update) -> bool {
        for rule in &self.rules {
            let aposo = update.iter().position(|page| *page == rule.0);
            let bposo = update.iter().position(|page| *page == rule.1);
            match (aposo,bposo) {
                (Some(apos),Some(bpos)) => {
                    if apos > bpos {
                        if VERBOSE { println!("incorrect: {} before {}", rule.1, rule.0); }
                        return false;
                    }
                },
                (_,_) => {}
            }
        }
        true
    }

    // must a come before b?
    fn cmp(&self, a:Page, b:Page) -> Ordering {
        for rule in &self.rules {
            if a == rule.0 && b == rule.1 {
                if VERBOSE { println!("  {} < {}", a, b);}
                return Ordering::Less;
            }
            if a == rule.1 && b == rule.0 {
                if VERBOSE { println!("  {} > {}", a, b);}
                return Ordering::Greater;
            }
        }
        if VERBOSE { println!("  {} = {}", a, b);}
        assert_eq!(a,b);
        Ordering::Equal
    }

}

//////////////////////////////////////////
/// Puzzle
//////////////////////////////////////////

struct Puzzle {
    rules:Rules,
    updates:Vec<Update>
}

impl Puzzle {

    fn get_middle_page(update:&Update) -> Page {
        // expect odd number of elements
        assert!(update.len() % 2 == 1);
        update[update.len() / 2]
    }

    fn sum_of_correct_middle_pages(&self) -> u32 {
        self.updates.iter().filter( |u| self.rules.is_correct_update(u)).map( |u| Self::get_middle_page(u)).sum()
    }

    fn fix_update(rules:&Rules, update:&mut Vec<Page>) {
        update.sort_by(|a,b| rules.cmp(*a,*b) );
    }

    fn fix_incorrect(&mut self) {
        for update in &mut self.updates {
            if ! self.rules.is_correct_update(&update) {
                if VERBOSE { println!("To sort: {:?}", update); }
                Self::fix_update(&self.rules, update);
                if VERBOSE { println!("Sorted: {:?}", update); }
            }
        }
    }
}

#[test]
fn test_puzzle_helper() {
    assert_eq!(Puzzle::get_middle_page(&vec![1,2,3,4,5]),3);
}

//////////////////////////////////////////
/// Parsing
//////////////////////////////////////////

fn read_puzzle(lines:Vec<String>) -> Puzzle {
    // section 1 = rules
    // section 2 = updates
    let mut section = 1;
    let mut rules = Vec::new();
    let mut updates = Vec::new();
    for line in lines {
        if VERBOSE { println!("{}", line); }
        if line == "" {
            section += 1;
        } else if section == 1 {
            let mut parts = line.split("|");
            let a = parts.next().unwrap().parse::<Page>().unwrap();
            let b = parts.next().unwrap().parse::<Page>().unwrap();
            assert!(parts.next().is_none());
            rules.push((a,b));
        } else if section == 2 {
            let parts = line.split(",");
            let update = parts.map(|s| s.parse::<Page>().unwrap()).collect();
            updates.push(update);
        }
    }
    Puzzle{rules:Rules{rules:rules}, updates}
}

#[cfg(test)]
fn input1() -> Vec<String> {
"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47".split("\n").map(|s| s.to_string()).collect()
}

#[test]
fn test_read_puzzle() {
    let puzzle = read_puzzle(input1());
    assert_eq!(puzzle.rules.rules.len(), 21);
    assert_eq!(puzzle.rules.rules[0], (47,53));
    assert_eq!(puzzle.updates.len(), 6);
    assert_eq!(puzzle.updates[0], vec![75,47,61,53,29]);
    assert_eq!(puzzle.rules.is_correct_update(&puzzle.updates[0]), true);
    assert_eq!(puzzle.rules.is_correct_update(&puzzle.updates[1]), true);
    assert_eq!(puzzle.rules.is_correct_update(&puzzle.updates[2]), true);
    assert_eq!(puzzle.rules.is_correct_update(&puzzle.updates[3]), false);
    assert_eq!(puzzle.rules.is_correct_update(&puzzle.updates[4]), false);
    assert_eq!(puzzle.rules.is_correct_update(&puzzle.updates[5]), false);
    assert_eq!(puzzle.sum_of_correct_middle_pages(), 143);

    let mut puzzle2 = puzzle;
    assert_eq!(puzzle2.rules.cmp(47, 53), Ordering::Less);
    assert_eq!(puzzle2.rules.cmp(13, 97), Ordering::Greater);
    puzzle2.fix_incorrect();
    assert_eq!(puzzle2.rules.is_correct_update(&puzzle2.updates[3]), true);
    assert_eq!(puzzle2.rules.is_correct_update(&puzzle2.updates[4]), true);
    assert_eq!(puzzle2.rules.is_correct_update(&puzzle2.updates[5]), true);
}

//////////////////////////////////////////
/// Real Puzzle
//////////////////////////////////////////

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn puzzle() {
    let file = File::open("input/day5.txt").expect("Could not open input/day5.txt");
    let reader = BufReader::new(file);

    let lines:Vec<String> = reader.lines().map( |line| line.unwrap() ).collect();

    let puzzle = read_puzzle(lines);
    let sum = puzzle.sum_of_correct_middle_pages();
    println!("Day 3, Part 1: Sum of middlepages of correct updates is {}", sum);

}