
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

    fn sum_of_multiplications(&self) -> u32 {
        self.multiplications.iter().map( |(a,b)| a * b).sum()
    }
}

#[test]
fn test_parser() {

    let parser1 = Parser::parse(vec!["limulbatrimul(22fimul(12,34)brmul(9999,12)".to_string()]);
    assert_eq!(parser1.multiplications, vec![(12,34)]);
    assert_eq!(parser1.sum_of_multiplications(), 12*34);

    let parser2 = Parser::parse(vec!["xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string()]);
    assert_eq!(parser2.multiplications, vec![(2,4),(5,5),(11,8),(8,5)]);
    assert_eq!(parser2.sum_of_multiplications(), 161);

}