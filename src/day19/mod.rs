use std::{
    collections::{hash_map::RandomState, HashMap},
    io::BufRead,
};

fn rules_match<'a>(
    rule_id: &Node,
    rules: &HashMap<usize, Node, RandomState>,
    line: &'a [u8],
) -> Vec<&'a [u8]> {
    if line.is_empty() {
        vec![]
    } else {
        match rule_id {
            Node::NonTerminal(a) => {
                if line[0].eq(a) {
                    vec![&line[1..]]
                } else {
                    vec![]
                }
            }
            Node::Terminal(b) => rules_match(rules.get(b).unwrap(), rules, line),
            Node::Or(a, b) => {
                let mut vec = rules_match(a, rules, line);
                vec.extend(rules_match(b, rules, line));
                vec
            }
            Node::And(a, b) => {
                let mut vec = vec![];
                for line in rules_match(a, rules, line) {
                    vec.extend(rules_match(b, rules, line))
                }
                vec
            }
        }
    }
}

#[derive(Clone)]
enum Node {
    Terminal(usize),
    NonTerminal(u8),
    And(Box<Node>, Box<Node>),
    Or(Box<Node>, Box<Node>),
}

fn parse_line(line: &str) -> (usize, Node) {
    let mut split = line.split(":");
    let rule_id = split.next().unwrap().parse().unwrap();
    (
        rule_id,
        split
            .next()
            .unwrap()
            .trim()
            .split(" | ")
            .fold(None, |end, p| {
                let node = if p.eq(r#""a""#) {
                    Node::NonTerminal('a' as u8)
                } else if p.eq(r#""b""#) {
                    Node::NonTerminal('b' as u8)
                } else {
                    p.split(" ")
                        .fold(None, |end, p| {
                            let node = Node::Terminal(p.parse().unwrap());
                            if let Some(m) = end {
                                Some(Node::And(Box::new(m), Box::new(node)))
                            } else {
                                Some(node)
                            }
                        })
                        .unwrap()
                };
                if let Some(m) = end {
                    Some(Node::Or(Box::new(m), Box::new(node)))
                } else {
                    Some(node)
                }
            })
            .unwrap(),
    )
}

pub fn run<'a>(reader: Box<dyn BufRead + 'a>) {
    let mut lines = reader.lines();
    let mut rules = HashMap::new();
    while let Some(input) = lines.next() {
        let line = input.unwrap();
        let line = line.trim();
        if line.is_empty() {
            break;
        }
        let (k, v) = parse_line(&line);
        rules.insert(k, v);
    }
    let mut new_rules = rules.clone();
    for line in ["8: 42 | 42 8", "11: 42 31 | 42 11 31"].iter() {
        let (k, v) = parse_line(line);
        new_rules.insert(k, v);
    }
    // new rules
    let mut count = 0;
    let mut new_count = 0;
    for line in lines {
        let line = line.unwrap();
        if rules_match(rules.get(&0).unwrap(), &rules, &line.as_bytes())
            .into_iter()
            .any(|m| m.is_empty())
        {
            count += 1
        }
        if rules_match(new_rules.get(&0).unwrap(), &new_rules, &line.as_bytes())
            .into_iter()
            .any(|m| m.is_empty())
        {
            new_count += 1
        }
    }
    println!("valid string = {}", count);
    println!("valid string = {}", new_count);
}
