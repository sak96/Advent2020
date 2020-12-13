use std::io::BufRead;
use std::collections::HashSet;

pub fn run<'a>(reader: Box<dyn BufRead + 'a>) {
    let mut hm: HashSet<u8> = HashSet::with_capacity(26);
    let mut hm1: HashSet<u8> = HashSet::with_capacity(26);
    let mut count = 0;
    let mut count1 = 0;
    let mut clear = true;
    for input in reader.lines() {
        let line = input.unwrap();
        if line.is_empty() {
            count += hm.len();
            count1 += hm1.len();
            hm.clear();
            hm1.clear();
            clear = true;
        }
        else {
            hm.extend(line.as_bytes());
            if clear{
                hm1.extend(line.as_bytes());
            } else {
                hm1 = hm1.intersection(&line.bytes().into_iter().collect::<HashSet<_>>()).cloned().collect();
            }
            clear = false;
        }
    }
    println!("{}", count);
    println!("{}", count1);
}
