use std::collections::HashSet;

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy)]
struct Computer
{
    name:[char;2]
}

impl std::fmt::Debug for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.name[0], self.name[1])
    }
}

impl Computer
{
    fn from_str(s:&str) -> Computer {
        assert_eq!(s.len(), 2);
        let mut chars = s.chars();
        let name_0 = chars.next().unwrap();
        let name_1 = chars.next().unwrap();
        Computer {
            name:[name_0, name_1]
        }
    }

    fn to_string(&self) -> String {
        String::from(format!("{}{}", self.name[0], self.name[1]))
    }
}

struct Network {
    computers:HashSet<Computer>,
    //links:HashMap<Computer, Vec<Computer>>
    links:Vec<(Computer, Computer)>
}

impl Network {
    fn links_from(&self, computer_from:Computer) -> Vec<Computer> {
        self.links.iter()
             .filter(|(computer1,computer2)| *computer1 == computer_from || *computer2 == computer_from )
             .map(|&(computer1,computer2)| if computer1 == computer_from { computer2 } else { computer1 }  )
             .collect()
    }
}

fn read_input<'a>(lines:impl Iterator<Item=&'a str>) -> Network {
    let mut computers:HashSet<Computer> = HashSet::new();
    //let mut links:HashMap<Computer, Vec<Computer>> = HashMap::new();
    let mut links:Vec<(Computer, Computer)> = Vec::new();
    for line in lines {
        let mut parts = line.split('-');
        let computer1 = Computer::from_str(parts.next().unwrap());
        let computer2 = Computer::from_str(parts.next().unwrap());
        assert!(parts.next().is_none());
        if computers.insert(computer1) {
            //links.insert(computer1, Vec::new());
        }
        if computers.insert(computer2) {
            //links.insert(computer2, Vec::new());
        }
        links.push((computer1, computer2));
        //links.push((computer2, computer1));
        //links.get(&computer1).as_mut().unwrap().push(computer2);
        //links.get(&computer2).unwrap().push(computer1);
    }
    Network { computers, links }
}

type SetOfThree = (Computer, Computer, Computer);

fn normalize_set_of_three(set:SetOfThree) -> SetOfThree {
    let mut vec = vec![set.0, set.1, set.2];
    vec.sort_by(|a,b| a.to_string().cmp(&b.to_string()));
    (vec[0], vec[1], vec[2])
}

#[test]
fn test_normalize() {
    assert_eq!(normalize_set_of_three((c("ab"), c("cd"), c("ef"))), (c("ab"), c("cd"), c("ef")));
    assert_eq!(normalize_set_of_three((c("cd"), c("ab"), c("ef"))), (c("ab"), c("cd"), c("ef")));
    assert_eq!(normalize_set_of_three((c("ef"), c("ab"), c("cd"))), (c("ab"), c("cd"), c("ef")));
}

fn find_sets_of_three(network:&Network) -> Vec<SetOfThree> {
    let mut sets : Vec<(Computer, Computer, Computer)> = Vec::new();
    for &computer1 in &network.computers {
        let linked1 = network.links_from(computer1);
        for computer2 in linked1 {
            let linked2 = network.links_from(computer2);
            for computer3 in linked2 {
                if computer3 == computer1 { continue; }
                let linked3 = network.links_from(computer3);
                if linked3.iter().find(|&&linked_computer| linked_computer == computer1).is_none() { continue;}
                let set = normalize_set_of_three((computer1,computer2,computer3));
                sets.push(set);
            }
        }
    }
    sets.sort();
    sets.dedup();
    sets
}

#[cfg(test)]
fn c(name:&str) -> Computer {
    Computer::from_str(name)
}

#[cfg(test)]
fn input1() -> &'static str {
"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"
}

#[test]
fn test_example1() {
    let input = input1();
    let lines = input.split('\n');
    let network = read_input(lines);
    let mut computers:Vec<String> = network.computers.iter().map(|computer| computer.to_string()).collect();
    computers.sort();
    assert_eq!(computers, vec!["aq", "cg", "co", "de", "ka", "kh", "qp", "ta", "tb", "tc", "td", "ub", "vc", "wh", "wq", "yn" ]);
    assert_eq!(network.links_from(c("de")),vec![c("cg"), c("co"), c("ta"), c("ka")]);

    let sets = find_sets_of_three(&network);
    assert_eq!(sets, vec![
        (c("aq"),c("cg"),c("yn")),
        (c("aq"),c("vc"),c("wq")),
        (c("co"),c("de"),c("ka")),
        (c("co"),c("de"),c("ta")),
        (c("co"),c("ka"),c("ta")),
        (c("de"),c("ka"),c("ta")),
        (c("kh"),c("qp"),c("ub")),
        (c("qp"),c("td"),c("wh")),
        (c("tb"),c("vc"),c("wq")),
        (c("tc"),c("td"),c("wh")),
        (c("td"),c("wh"),c("yn")),
        (c("ub"),c("vc"),c("wq"))
    ]);
}