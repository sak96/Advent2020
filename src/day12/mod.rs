use std::io::BufRead;

enum Direction {
    East,
    West,
    North,
    South,
}

struct Boat {
    east: i64,
    north: i64,
    direction: Direction,
}

struct WayPoint {
    east: i64,
    north: i64,
    boat: Boat
}

impl WayPoint {
    pub fn new() -> Self {
        Self {
            east: 10,
            north: 1,
            boat: Boat::new()
        }
    }

    pub fn move_forward(&mut self, distance: i64) {
        self.boat.east += distance * self.east;
        self.boat.north += distance * self.north;
    }

    pub fn move_instr(&mut self, instr: &str) {
        let distance = instr[1..].parse::<i64>().unwrap();
        match instr.chars().nth(0).unwrap() {
            'E' => self.east += distance,
            'W' => self.east -= distance,
            'N' => self.north += distance,
            'S' => self.north -= distance,
            'F' => self.move_forward(distance),
            'R' => match distance {
                0 => {},
                90 => self.turn_right(),
                180 => {self.turn_right(); self.turn_right()}
                270 => {self.turn_right(); self.turn_right(); self.turn_right()}
                _ => unreachable!()
            }
            'L' => match distance {
                0 => {},
                90 => {self.turn_right(); self.turn_right(); self.turn_right()}
                180 => {self.turn_right(); self.turn_right()}
                270 => self.turn_right(),
                _ => unreachable!()
            }
            _ => unreachable!(),
        }
    }

    fn turn_right(&mut self) {
        std::mem::swap(&mut self.east, &mut self.north);  
        self.north = -self.north
    }

    pub fn manhattan_distance(&self) -> i64 {
        self.boat.manhattan_distance()
    }
}

impl Boat {
    pub fn new() -> Self {
        Self {
            east: 0,
            north: 0,
            direction: Direction::East,
        }
    }

    fn turn_right(&mut self) {
        use Direction::*;
        self.direction = match self.direction {
            East => South,
            West => North,
            South => West,
            North => East,
        }
    }
    fn move_dist_east(&mut self, distance: i64) {
        self.east += distance
    }

    fn move_dist_north(&mut self, distance: i64) {
        self.north += distance
    }

    pub fn move_instr(&mut self, instr: &str) {
        use Direction::*;
        let distance = instr[1..].parse::<i64>().unwrap();
        match instr.chars().nth(0).unwrap() {
            'E' => self.move_dist_east(distance),
            'W' => self.move_dist_east(-distance),
            'N' => self.move_dist_north(distance),
            'S' => self.move_dist_north(-distance),
            'F' => match self.direction {
                East => self.move_dist_east(distance),
                West => self.move_dist_east(-distance),
                North => self.move_dist_north(distance),
                South => self.move_dist_north(-distance),
            },
            'R' => match distance {
                0 => {},
                90 => self.turn_right(),
                180 => {self.turn_right(); self.turn_right()}
                270 => {self.turn_right(); self.turn_right(); self.turn_right()}
                _ => unreachable!()
            }
            'L' => match distance {
                0 => {},
                90 => {self.turn_right(); self.turn_right(); self.turn_right()}
                180 => {self.turn_right(); self.turn_right()}
                270 => self.turn_right(),
                _ => unreachable!()
            }
            _ => unreachable!(),
        }
    }

    pub fn manhattan_distance(&self) -> i64 {
        self.east.abs() + self.north.abs()
    }
}

pub fn run<'a>(reader: Box<dyn BufRead + 'a>) {
    let mut boat = Boat::new();
    let mut way_point = WayPoint::new();
    for input in reader.lines() {
        let line = input.unwrap();
        let line = line.trim();
        boat.move_instr(line);
        way_point.move_instr(line);
        println!("man hattan distance {}:{}", way_point.boat.east, way_point.boat.north);
    }
    println!("man hattan distance {}", boat.manhattan_distance());
    println!("man hattan distance {}", way_point.manhattan_distance());
}
