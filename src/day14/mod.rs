use std::collections::HashMap;
use std::io::BufRead;

pub fn run<'a>(reader: Box<dyn BufRead + 'a>) {
    let mut hm = HashMap::new();
    let mut hm1 = HashMap::new();
    let mut and_mask = 0u64;
    let mut or_mask = std::u64::MAX;

    for input in reader.lines() {
        let line = input.unwrap();
        if line.starts_with("mask") {
            or_mask = 0u64;
            and_mask = 0u64;
            // mask     = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
            // and_mask = 111111111111111111111111111111111101
            // or_mask  = 000000000000000000000000000001000000
            let mask = line.split("=").nth(1).unwrap().trim();
            for i in mask.chars() {
                and_mask <<= 1;
                or_mask <<= 1;
                match i {
                    '1' => {
                        or_mask |= 1;
                        and_mask |= 1;
                    }
                    '0' => {}
                    'X' => {
                        and_mask |= 1;
                    }
                    _ => unreachable!(format!("invalid mask character {}", i)),
                }
            }
        } else {
            // mem[8] = 11
            let mut split = line.split(" = ");
            let loc_word = split.next().unwrap();
            let value = split.next().unwrap().trim().parse::<u64>().unwrap();
            let loc = loc_word
                .chars()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
                .parse::<u64>()
                .unwrap();
            hm.insert(loc, (value | or_mask) & and_mask);
            // set bit to all memory address
            {
                // mask == 1 at X and 0 at non X
                let mut mask = or_mask ^ and_mask;
                // base lock is or with or_mask
                let loc = loc | or_mask;
                let mut vec = vec![loc];
                hm1.insert(loc, value);
                let mut p = 1;
                while mask != 0 {
                    if mask & 1 != 0 {
                        let temp: Vec<_> = vec.iter().cloned().collect();
                        for j in temp {
                            hm1.insert(j ^ p, value);
                            vec.push(j ^ p);
                        }
                    }
                    mask >>= 1;
                    p <<= 1;
                }
            }
        }
    }
    let sum: u64 = hm.iter().map(|p| p.1).fold(0u64, |sum, &a| sum + a as u64);
    println!("sum of loc {}", sum);
    let sum: u64 = hm1.iter().map(|p| p.1).fold(0u64, |sum, &a| sum + a as u64);
    println!("sum of loc {}", sum);
}
