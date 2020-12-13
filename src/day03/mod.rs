use std::io::BufRead;

#[derive(Debug)]
struct Slope {
    right: usize,
    down: usize,
    cur_x: usize,
    cur_y: usize,
    tree: usize,
}

impl Slope {
    pub fn new(right: usize, down: usize) -> Self {
        Self {
            right,
            down,
            cur_x: 0,
            cur_y: 0,
            tree: 0,
        }
    }

    pub fn update_tree(&mut self, input: &str) {
        self.cur_y %= self.down;
        dbg!(self.cur_y, self.down);
        if self.cur_y == 0 {
            self.cur_x %= input.len();
            if matches!(input.chars().nth(self.cur_x), Some('#')) {
                self.tree += 1;
            }
            self.cur_x += self.right;
        }
        self.cur_y += 1;
    }
}

pub fn run<'a>(reader: Box<dyn BufRead + 'a>) {
    let right = [1, 3, 5, 7, 1];
    let down = [1, 1, 1, 1, 2];
    let mut slopes: Vec<_> = right
        .iter()
        .zip(down.iter())
        .map(|(&right, &down)| Slope::new(right, down))
        .collect();

    for input in reader.lines() {
        let input = input.unwrap();
        for slope in slopes.iter_mut() {
            slope.update_tree(&input);
        }
    }
    println!("{}", slopes[1].tree);
    println!("Tree mul={}", slopes.into_iter().fold(1, |a, b| a * b.tree));
}
