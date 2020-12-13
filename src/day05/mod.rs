use std::io::BufRead;

//  FBFBBFF 44  RLR 5
pub fn run<'a>(reader: Box<dyn BufRead + 'a>) {
    let mut vec = vec![];
    for input in reader.lines() {
        let s = input.unwrap();
        let mut num = 0;
        for i in s.chars() {
            num = if i == 'B' || i == 'R' {
                num * 2 + 1
            } else {
                num * 2
            };
        }
        vec.push(num);
    }
    println!("max: {}", vec.iter().max().unwrap());
    vec.sort();
    println!("my seat: {:?}",vec.as_slice().windows(2).filter(|p| p[0] + 2 == p[1] ).next().unwrap()[0] + 1);
}
