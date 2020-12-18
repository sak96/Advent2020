use std::collections::HashSet;
use std::io::BufRead;

pub fn run<'a>(reader: Box<dyn BufRead + 'a>) {
    let mut active = HashSet::new();
    let mut active_w = HashSet::new();
    let mut max_x: i16 = 0;
    let mut max_y: i16 = 0;
    for (y, input) in reader.lines().enumerate() {
        for (x, c) in input.unwrap().trim_end().chars().enumerate() {
            if c == '#' {
                active.insert((x as i16, y as i16, 0));
                active_w.insert((x as i16, y as i16, 0, 0));
            }
            max_x = max_x.max(x as i16);
            max_y = max_y.max(y as i16);
        }
    }

    {
        let mut max_x = max_x;
        let mut max_y = max_y;
        let mut min_x = 0;
        let mut min_y = 0;
        let mut max_z = 0;
        let mut min_z = 0;
        for _ in 0..6 {
            let mut new_active = HashSet::new();
            min_x -= 1;
            max_x += 1;
            min_y -= 1;
            max_y += 1;
            min_z -= 1;
            max_z += 1;
            for z in min_z..=max_z {
                for y in min_y..=max_y {
                    for x in min_x..=max_x {
                        let mut count = 0;
                        for dx in -1..2 {
                            for dy in -1..2 {
                                for dz in -1..2 {
                                    if active.contains(&(x + dx, y + dy, z + dz)) {
                                        count += 1;
                                    }
                                }
                            }
                        }
                        if active.contains(&(x, y, z)) && (count == 3 || count == 4) {
                            // cube = active if active_neg in [2,3] and self.is_active
                            // count is 3 and 4 as we count self
                            new_active.insert((x, y, z));
                        } else if !active.contains(&(x, y, z)) && (count == 3) {
                            // cube = active if active_neg in [3] and self.is_active
                            new_active.insert((x, y, z));
                        }
                    }
                }
            }
            active = new_active;
        }

        println!("total active in 3d {}", active.len());
    }

    {
        let mut max_x = max_x;
        let mut max_y = max_y;
        let mut min_x: i16 = 0;
        let mut min_y: i16 = 0;
        let mut max_z: i16 = 0;
        let mut min_z: i16 = 0;
        let mut max_w: i16 = 0;
        let mut min_w: i16 = 0;
        for _ in 0..6 {
            let mut new_active = HashSet::new();
            min_x -= 1;
            max_x += 1;
            min_y -= 1;
            max_y += 1;
            min_z -= 1;
            max_z += 1;
            min_w -= 1;
            max_w += 1;
            for w in min_w..=max_w {
                for z in min_z..=max_z {
                    for y in min_y..=max_y {
                        for x in min_x..=max_x {
                            let mut count = 0;
                            for dx in -1..2 {
                                for dy in -1..2 {
                                    for dz in -1..2 {
                                        for dw in -1..2 {
                                            if active_w.contains(&(x + dx, y + dy, z + dz, w + dw))
                                            {
                                                count += 1;
                                            }
                                        }
                                    }
                                }
                            }
                            if active_w.contains(&(x, y, z, w)) && (count == 3 || count == 4) {
                                // cube = active if active_neg in [2,3] and self.is_active
                                // count is 3 and 4 as we count self
                                new_active.insert((x, y, z, w));
                            } else if !active_w.contains(&(x, y, z, w)) && (count == 3) {
                                // cube = active if active_neg in [3] and self.is_active
                                new_active.insert((x, y, z, w));
                            }                        }
                    }
                }
            }
            active_w = new_active;
        }

        println!("total active in 4d {}", active_w.len());
    }
}
