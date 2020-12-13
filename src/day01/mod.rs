use std::io::BufRead;

fn find_sum_for_two(mut vec: &[u64], limit: u64) -> u64 {
    let mut sum = 0;
    loop {
        if vec.len() < 2 || vec[0] + vec[1] > limit {
            break;
        }
        if let Ok(other) = vec.binary_search(&(limit - vec[0])) {
            sum += vec[other] * vec[0]
        }
        vec = &vec[1..];
    }
    sum
}

pub fn run<'a>(reader: Box<dyn BufRead + 'a>) {
    let mut vec = vec![];
    for input in reader.lines() {
        vec.push(input.unwrap().trim_end().parse::<u64>().unwrap());
    }
    vec.sort();
    let mut vec = vec.as_slice();

    // solution 1
    println!("{}", find_sum_for_two(vec, 2020));

    // solution 1
    {
        let mut sum = 0;
        loop {
            if vec.len() < 3 || vec[0] + vec[1] + vec[2] > 2020 {
                break;
            }
            let num = vec[1];
            vec = &vec[1..];
            sum += num * find_sum_for_two(vec, 2020 - num);
        }
        println!("{}", sum);
    }
}
