use std::io::BufRead;

pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn inverse_modulo(a: usize, c: usize, b: usize) -> usize {
    for i in 0..=b {
        if (i * a) % b == c {
            return i;
        }
    }
    unreachable!()
}

pub fn chinese_remainder(equation: &[(usize, usize)]) -> usize {
    let big_m: usize = equation.iter().map(|a| a.1).product();
    equation
        .iter()
        .map(|eq| (big_m / eq.1) * inverse_modulo((big_m / eq.1) % eq.1, eq.0, eq.1))
        .sum::<usize>()
        % big_m
}

pub fn run<'a>(reader: Box<dyn BufRead + 'a>) {
    let mut max_time = std::usize::MAX;
    let mut max_bus_num = None;
    let mut lines = reader.lines();
    let start = lines.next().unwrap().unwrap();
    let start = start.trim().parse::<usize>().unwrap();
    let line = lines.next().unwrap().unwrap();
    let line = line.trim();

    let mut equation = vec![];
    for (i, item) in line.split(",").enumerate() {
        if item == "x" {
            continue;
        }
        let bus_num = item.parse::<usize>().unwrap();
        let wait_time = bus_num - (start % bus_num);
        if wait_time < max_time {
            max_time = wait_time;
            max_bus_num = Some(bus_num);
        }
        equation.push(((bus_num + i * bus_num - i) % bus_num, bus_num));
    }
    if let Some(bus_num) = max_bus_num {
        println!("wait_time * bus_num: {}", bus_num * max_time);
    }
    println!("base_wait_time is : {}", chinese_remainder(&equation));
}
