use std::collections::HashSet;
use std::io::BufRead;

pub fn run<'a>(reader: Box<dyn BufRead + 'a>) {
    let mut vec = vec![];
    let mut pc = 0;
    let mut acc = 0;
    let mut hm = HashSet::new();

    for input in reader.lines() {
        let line = input.unwrap();
        vec.push(line);
    }

    // nop +0
    // acc +1
    // jmp +4
    // acc +3
    // jmp -3
    // acc -99
    // acc +1
    // jmp -4
    // acc +6
    while hm.insert(pc) {
        let line = vec[pc as usize].as_str();
        let mut split = line.trim().split_whitespace();
        let operator = split.next().unwrap();
        let operand: i64 = split.next().unwrap().parse().unwrap();
        match operator {
            "nop" => pc += 1,
            "jmp" => pc += operand,
            "acc" => {
                pc += 1;
                acc += operand;
            }
            _ => unreachable!("{} not a valid operator", operator),
        };
    }
    println!("acc before loop {}", acc);

    let mut temp = String::new();
    for i in 0..vec.len() {
        hm.clear();
        pc = 0;
        acc = 0;
        std::mem::swap(&mut temp, &mut vec[i as usize]);
        if temp.starts_with("jmp") {
            vec[i as usize] = temp.replace("jmp", "nop");
        } else if temp.starts_with("nop") {
            vec[i as usize] = temp.replace("nop", "jmp");
        } else {
            std::mem::swap(&mut temp, &mut vec[i as usize]);
            continue;
        };
        while hm.insert(pc) {
            let line = vec[pc as usize].as_str();
            let mut split = line.trim().split_whitespace();
            let operator = split.next().unwrap();
            let operand: i64 = split.next().unwrap().parse().unwrap();
            match operator {
                "nop" => pc += 1,
                "jmp" => pc += operand,
                "acc" => {
                    pc += 1;
                    acc += operand;
                }
                _ => unreachable!("{} not a valid operator", operator),
            };
            if pc as usize == vec.len() {
                println!("{} swapped to exit properly", acc);
                break;
            };
        }
        std::mem::swap(&mut temp, &mut vec[i as usize]);
    }
}
