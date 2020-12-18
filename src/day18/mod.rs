use std::io::BufRead;

pub fn run<'a>(reader: Box<dyn BufRead + 'a>) {
    let mut sum = 0;
    let mut sum_add = 0;
    for input in reader.lines() {
        let line = input.unwrap();
        sum += evaluate(&line);
        sum_add += evaluate_add(&line);
    }
    println!("{}", sum);
    println!("{}", sum_add);
}

#[derive(Debug)]
enum Token {
    Bracket,
    Add,
    Mul,
    Operand(usize),
}

fn evaluate(line: &str) -> usize {
    let mut stack = vec![];
    for i in line.split(' ') {
        if i.starts_with('+') {
            stack.push(Token::Add);
        } else if i.starts_with('*') {
            stack.push(Token::Mul);
        } else {
            let mut i = i;
            let mut unwrap = 1;
            while i.starts_with('(') {
                stack.push(Token::Bracket);
                i = &i[1..];
            }
            while i.ends_with(')') {
                i = &i[..i.len() - 1];
                unwrap += 1;
            }
            let mut result = i.parse().unwrap();
            for _ in 0..unwrap {
                match stack.pop() {
                    Some(Token::Add) => {
                        if let Some(Token::Operand(oth)) = stack.pop() {
                            result += oth
                        }
                    }
                    Some(Token::Mul) => {
                        if let Some(Token::Operand(oth)) = stack.pop() {
                            result *= oth
                        }
                    }
                    _ => {}
                }
            }
            stack.push(Token::Operand(result));
        }
    }
    match stack.pop() {
        Some(Token::Operand(a)) => a,
        _ => unreachable!("this should not be the case"),
    }
}

fn evaluate_add(line: &str) -> usize {
    let mut stack = vec![];
    for i in line.split(' ') {
        if i.starts_with('+') {
            stack.push(Token::Add);
        } else if i.starts_with('*') {
            stack.push(Token::Mul);
        } else {
            let mut i = i;
            let mut unwrap = 0;
            while i.starts_with('(') {
                stack.push(Token::Bracket);
                i = &i[1..];
            }
            while i.ends_with(')') {
                i = &i[..i.len() - 1];
                unwrap += 1;
            }
            let mut result = i.parse().unwrap();
            for _ in 0..unwrap {
                loop {
                    match stack.pop() {
                        Some(Token::Add) => {
                            if let Some(Token::Operand(oth)) = stack.pop() {
                                result += oth
                            }
                        }
                        Some(Token::Mul) => {
                            if let Some(Token::Operand(oth)) = stack.pop() {
                                result *= oth
                            }
                        }
                        _ => {break}
                    }
                }
            }
            match stack.pop() {
                Some(Token::Add) => {
                    if let Some(Token::Operand(oth)) = stack.pop() {
                        result += oth
                    }
                }
                Some(p) => {
                    stack.push(p);
                }
                None => {}
            }
            stack.push(Token::Operand(result));
        }
    }
    match stack.pop() {
        Some(Token::Operand(mut result)) => {
            while stack.pop().is_some() {
                if let Some(Token::Operand(oth)) = stack.pop() {
                    result *= oth
                }
            }
            result
        }
        _ => unreachable!("this should not be the case"),
    }
}
