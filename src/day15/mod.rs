use std::collections::HashMap;
use std::io::BufRead;

pub fn run<'a>(reader: Box<dyn BufRead + 'a>) {
    let line = reader.lines().next().unwrap().unwrap();
    let split = line.trim().split(',');
    let mut hm: HashMap<_,_> = split.into_iter().enumerate().map(|(i, num)| (num.parse::<usize>().unwrap(), i+1)).collect();
    let mut next_number = 0;

    for turn in (hm.len()+1)..30000000{
        let new_next: usize;
        if let Some(old) = hm.get(&next_number){
            new_next = turn-old;
        } else {
            new_next = 0
        }
        hm.insert(next_number,turn);
        if turn == 2020{
            println!("{}", next_number);
        }
        next_number = new_next;
    }
    println!("{}", next_number);

}
