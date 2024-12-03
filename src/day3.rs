
//////////////////////////////////////////
/// Parser
//////////////////////////////////////////

type Multiplication = (u32, u32);
type Multiplications = Vec<Multiplication>;

struct Parser {
    lines:Vec<String>,
    multiplications:Multiplications
}

impl Parser {
    fn parse_line(line:&str, multiplications:&mut Multiplications) {
        let mut parts = line.split("mul(");
        parts.next(); // ignore everything before first "mul"
        for part in parts {
            print!("mul({}: ", &part);

            let rbracket_split = part.split_once(")");
            if rbracket_split.is_none() {
                println!("no ')'");
                continue;
            }

            let Some((a_comma_b, _rbracket)) = rbracket_split else { panic!("{:?}", rbracket_split)};
            let comma_split = a_comma_b.split_once(",");
            if comma_split.is_none() {
                println!("no ','");
                continue;
            }

            let Some((a_str, b_str)) = comma_split else { panic!("{:?}", comma_split)};
            let a = match a_str.parse::<u32>() {
                Ok(a) => a,
                Err(_) => {
                    println!("{} is no number", a_str);
                    continue;
                }
            };
            if a > 999 {
                println!("{} is too big", a);
                continue;
            }

            let b = match b_str.parse::<u32>() {
                Ok(b) => b,
                Err(_) => {
                    println!("{} is no number", b_str);
                    continue;
                }
            };
            if b > 999 {
                println!("{} is too big", b);
                continue;
            }

            println!("ok");

            multiplications.push((a,b));
        }
    }

    fn parse(lines:Vec<String>) -> Parser {
        let mut multiplications:Multiplications = Multiplications::new();
        for line in &lines {
            Self::parse_line(line, &mut multiplications);
        }
        Parser { lines:lines, multiplications:multiplications }
    }
}

#[test]
fn test_parser() {

    let parser = Parser::parse(vec!["limulbatrimul(22fimul(12,34)brmul(9999,12)".to_string()]);
    assert_eq!(parser.multiplications, vec![(12,34)]);
}