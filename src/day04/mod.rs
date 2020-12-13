use std::io::BufRead;

// byr (Birth Year)
// iyr (Issue Year)
// eyr (Expiration Year)
// hgt (Height)
// hcl (Hair Color)
// ecl (Eye Color)
// pid (Passport ID)
// cid (Country ID)

pub fn run<'a>(reader: Box<dyn BufRead + 'a>) {
    const FELIDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    const EYE_COLOR: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    let mut vec1 = Vec::<String>::with_capacity(FELIDS.len());
    let mut vec2 = Vec::<String>::with_capacity(FELIDS.len());
    let mut valid1 = 0;
    let mut valid2 = 0;
    for input in reader.lines() {
        let s = input.unwrap();
        let s = s.trim();
        if s.is_empty() {
            if FELIDS.iter().all(|&a| vec1.contains(&a.to_string())) {
                valid1 += 1;
            }
            if FELIDS.iter().all(|&a| vec2.contains(&a.to_string())) {
                valid2 += 1;
            }
            vec1.clear();
            vec2.clear();
        } else {
            // byr (Birth Year) - four digits; at least 1920 and at most 2002.
            // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
            // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
            // hgt (Height) - a number followed by either cm or in:
            //     If cm, the number must be at least 150 and at most 193.
            //     If in, the number must be at least 59 and at most 76.
            // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
            // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
            // pid (Passport ID) - a nine-digit number, including leading zeroes.
            // cid (Country ID) - ignored, missing or not.
            //
            for both in s.split_whitespace() {
                let key = both.split(':').nth(0).unwrap().to_string();
                let value = both.split(':').nth(1).unwrap().to_string();
                if (key == "byr" && check_int_range(&value, 1920, 2002))
                    || (key == "iyr" && check_int_range(&value, 2010, 2020))
                    || (key == "eyr" && check_int_range(&value, 2020, 2030))
                    || (key == "hgt" && match_hgt(&value))
                    || (key == "hcl" && match_hcl(&value))
                    || (key == "ecl" && EYE_COLOR.contains(&value.as_str()))
                    || (key == "pid" && value.len() == 9 && value.parse::<u64>().is_ok())
                {
                    vec2.push(key.to_string())
                }
                vec1.push(key.to_string())
            }
        }
    }
    println!("valid passports with all fields except cid {}", valid1);
    println!("valid passports with validation {}", valid2);
}

pub fn match_hgt(value: &String) -> bool {
    (value.len() == 5 && value[3..].eq("cm") && check_int_range(&value[..3], 150, 193))
        || (value.len() == 4 && value[2..].eq("in") && check_int_range(&value[..2], 59, 76))
}

pub fn match_hcl(value: &String) -> bool {
    value.len() == 7
        && value.chars().nth(0).unwrap() == '#'
        && value[1..].chars().all(|p| p.is_digit(16))
}

pub fn check_int_range(value: &str, min: u32, max: u32) -> bool {
    if let Ok(num) = value.parse::<u32>() {
        if num >= min && num <= max {
            return true;
        }
    }
    false
}
