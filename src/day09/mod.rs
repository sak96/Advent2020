use std::io::BufRead;

fn find_odd_one(input: &[usize], preamble: usize) -> usize{
    for i in (preamble)..input.len() {
        let p = input[i-preamble..i].as_ref();
        let sum = input[i];
        let mut done = false;
        for i in 0..p.len() {
            for j in (i+1)..p.len() {
                if p[i] + p[j] == sum {
                    done = true;
                    break;
                }
            }
        }
            if !done {
                return sum
            }
    }
    unreachable!("this should not be reached");

}


pub fn run<'a>(reader: Box<dyn BufRead + 'a>) {
    let mut vec = vec![];
    for input in reader.lines() {
        let line = input.unwrap();
        vec.push(line.trim().parse::<usize>().unwrap());
    }

    let preamble = 25;
    let odd_one = find_odd_one(&vec, preamble);
    println!("the first odd one is {}", odd_one);

    let mut low = 0;
    let mut sum = 0;

    for i in 0..vec.len() {
        sum += vec[i];
        while sum > odd_one {
            sum -= vec[low];
            low +=1;
        }
        if sum == odd_one {
            let range = vec[low..i+1].as_ref();
            println!("the weakness is {}", range.iter().max().unwrap() + range.iter().min().unwrap());
            return
        }
    }

}
