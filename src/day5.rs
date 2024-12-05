const VERBOSE:bool = false;

type Page = u32;

// Page a must be before page b
type Rule = (/* a: */Page,/* b: */Page);

type Update = Vec<Page>;

struct Puzzle {
    rules:Vec<Rule>,
    updates:Vec<Update>
}

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
    Puzzle{rules, updates}
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
    assert_eq!(puzzle.rules.len(), 21);
    assert_eq!(puzzle.rules[0], (47,53));
    assert_eq!(puzzle.updates.len(), 6);
    assert_eq!(puzzle.updates[0], vec![75,47,61,53,29]);
}
