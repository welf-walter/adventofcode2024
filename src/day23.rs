use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Computer
{
    name:[char;2]
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
    assert_eq!(network.computers = vec!["aq", "cg", "co", "de", "ka", "kh", "qp", "ta", "tb", "tc", "td", "ub", "vc", "wh", "wq", "yn" ]);
    assert_eq!(network.links_from(c("de")),vec![c("cg"), c("co"), c("ta"), c("ka")]);
}