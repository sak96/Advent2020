use std::{collections::{HashSet, hash_map::RandomState}, io::BufRead};
use std::collections::HashMap;

pub fn run<'a>(reader: Box<dyn BufRead + 'a>) {
    let mut hm = HashMap::new();
    let mut hm1 = HashMap::new();
    for input in reader.lines() {
        let line = input.unwrap();

        // dotted black bags contain no other bags.
        // muted gray bags contain 2 striped crimson bags, 2 striped maroon bags, 1 faded maroon bag, 4 dim lavender bags.
        let mut split = line.splitn(2, "contain");
        let container = split.next().unwrap().trim().trim_end_matches("s");
        let contained = split.next().unwrap();
        let contained = contained.trim_end_matches(".");
        let bags = contained.split(",");
        for bag in bags{
            // 2 striped crimson bags,
            let mut split = bag.trim().splitn(2, " ");
            let amt: usize = split.next().unwrap().trim().parse().unwrap_or(0);
            let bag = split.next().unwrap().trim_end_matches("s");
            hm.entry(bag.to_string()).or_insert(vec![]).push((container.to_string(), amt));
            hm1.entry(container.to_string()).or_insert(vec![]).push((bag.to_string(), amt));
        }
    }

    let mut visited_bag = HashSet::new();
    let mut new_bags = HashSet::new();
    visited_bag.insert("shiny gold bag");
    new_bags.insert("shiny gold bag");

    while !new_bags.is_empty() {
        let mut new_new_bags = HashSet::new();
        for bag in new_bags.drain(){
            for bags in hm.get(bag){
                for bag in bags {
                    if visited_bag.insert(bag.0.as_str()) {
                        new_new_bags.insert(bag.0.as_str());
                    }
                }
            }
        }
        new_bags = new_new_bags;
    }
    println!("elements which can contain shiny gold bag are {}", visited_bag.len() - 1);

    println!("shiny gold bag must contain {}", get_bag_nested(&hm1, "shiny gold bag") - 1);
}

pub fn get_bag_nested(hm1: &HashMap<String, Vec<(String, usize)>, RandomState>, bag: &str) -> usize {
   hm1.get(bag).unwrap_or(&vec![]).iter().map(|(bag, amt)| amt *get_bag_nested(hm1,bag.as_str()) ).sum::<usize>() + 1
}
