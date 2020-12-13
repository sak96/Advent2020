use std::io::BufRead;

#[derive(Debug)]
struct Password {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl From<String> for Password {
    // 1-3 a: abcde
    fn from(input: String) -> Self {
        let mut slice = input.split_whitespace();
        let limits = slice.next().unwrap();
        let mut limits = limits.split("-");
        let min = limits.next().unwrap().parse().unwrap();
        let max = limits.next().unwrap().parse().unwrap();
        let letter = slice.next().unwrap().chars().nth(0).unwrap();
        let password = slice.next().unwrap().into();
        Self {
            max,
            min,
            letter,
            password,
        }
    }
}

impl Password {
    pub fn is_valid_count(&self) -> bool {
        let count = self.password.chars().filter(|&c| c == self.letter).count();
        count >= self.min && count <= self.max
    }
    pub fn is_valid_position(&self) -> bool {
        (self.password.chars().nth(self.min - 1) == Some(self.letter))
            ^ (self.password.chars().nth(self.max - 1) == Some(self.letter))
    }
}

pub fn run<'a>(reader: Box<dyn BufRead + 'a>) {
    let mut valid_password_count = 0;
    let mut valid_password_position = 0;
    for input in reader.lines() {
        if let Ok(input) = input {
            if !input.is_empty() {
                let password = Password::from(input);
                if password.is_valid_count() {
                    valid_password_count += 1;
                }
                if password.is_valid_position() {
                    valid_password_position += 1;
                }
            }
        }
    }

    println!("{}", valid_password_count);
    println!("{}", valid_password_position);
}
