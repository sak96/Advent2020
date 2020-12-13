use std::io::BufRead;

fn count_filled(slice: &[Seat]) -> usize {
    slice.iter().fold(0, |count, c| {
        if matches!(c, Seat::Filled) {
            count + 1
        } else {
            count
        }
    })
}

fn display_seats(vec: &[Seat], len: usize) {
    for (i, ch) in vec.iter().enumerate() {
        // print
        if i % len == 0 {
            println!("");
        }
        print!(
            "{}",
            match ch {
                Seat::Filled => '#',
                Seat::Empty => 'L',
                Seat::Floor => '.',
            }
        )
    }

    println!("");
    println!("---");
}

#[derive(Clone, Debug)]
enum Seat {
    Floor,
    Empty,
    Filled,
}

pub fn run<'a>(reader: Box<dyn BufRead + 'a>) {
    let mut vec = vec![];
    let len = 91;
    for input in reader.lines() {
        let line = input.unwrap();
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        assert_eq!(line.len(), len);
        vec.extend(
            line.chars()
                .map(|c| if c == 'L' { Seat::Empty } else { Seat::Floor }),
        );
    }
    // solution 1
    {
        let mut vec = vec.clone();
        let mut temp = vec.clone();

        let mut swapped = true;
        while swapped {
            swapped = false;
            std::mem::swap(&mut temp, &mut vec);

            for (i, ch) in vec.iter_mut().enumerate() {
                let mut count = 0;
                let col = i % len;
                let row = i - col;
                let col_min = col.saturating_sub(1);
                let col_max = (col + 1).min(len - 1);
                if row >= len {
                    count += count_filled(&temp[(row - len + col_min)..(row - len + col_max) + 1]);
                }
                if row + len < temp.len() {
                    count += count_filled(&temp[(row + len + col_min)..(row + len + col_max) + 1]);
                }
                count += count_filled(&temp[(row + col_min)..(row + col_max) + 1]);
                if matches!(temp[i], Seat::Filled) && count > 4 {
                    // dbg!(count, i);
                    swapped = true;
                    *ch = Seat::Empty;
                } else if matches!(temp[i], Seat::Empty) && count == 0 {
                    swapped = true;
                    *ch = Seat::Filled;
                } else {
                    *ch = temp[i].clone();
                }
            }
        }

        display_seats(&vec, len);
        println!("seat that are filled: {}", count_filled(&vec));
    }

    //solution 2
    {
        let mut vec = vec.clone();
        let mut temp = vec.clone();

        let mut swapped = true;
        while swapped {
            swapped = false;
            std::mem::swap(&mut temp, &mut vec);

            for (i, ch) in vec.iter_mut().enumerate() {
                let mut count = 0;
                let col = i % len;
                let row = i / len;
                if matches!(ch, Seat::Floor) {
                    continue;
                }

                // check top left
                {
                    let mut col = col;
                    let mut row = row;
                    while row > 0 && col > 0 {
                        row -= 1;
                        col -= 1;
                        match temp[row * len + col] {
                            Seat::Floor => continue,
                            Seat::Filled => {
                                count += 1;
                            }
                            Seat::Empty => {}
                        }
                        break;
                    }
                }

                // check top
                {
                    let mut row = row;
                    while row > 0 {
                        row -= 1;
                        match temp[row * len + col] {
                            Seat::Floor => continue,
                            Seat::Filled => {
                                count += 1;
                            }
                            Seat::Empty => {}
                        }
                        break;
                    }
                }

                // top right
                {
                    let mut col = col;
                    let mut row = row;
                    // check top left
                    while row > 0 && col + 1 < len {
                        row -= 1;
                        col += 1;
                        match temp[row * len + col] {
                            Seat::Floor => continue,
                            Seat::Filled => {
                                count += 1;
                            }
                            Seat::Empty => {}
                        }
                        break;
                    }
                }

                // left
                {
                    let mut col = col;
                    // check top left
                    while col > 0 {
                        col -= 1;
                        match temp[row * len + col] {
                            Seat::Floor => continue,
                            Seat::Filled => {
                                count += 1;
                            }
                            Seat::Empty => {}
                        }
                        break;
                    }
                }

                // right
                {
                    let mut col = col;
                    // check top left
                    while col + 1 < len {
                        col += 1;
                        match temp[row * len + col] {
                            Seat::Floor => continue,
                            Seat::Filled => {
                                count += 1;
                            }
                            Seat::Empty => {}
                        }
                        break;
                    }
                }

                let height = temp.len() / len;
                // bottom left
                {
                    let mut col = col;
                    let mut row = row;
                    // check top left
                    while row + 1 < height && col > 0 {
                        row += 1;
                        col -= 1;
                        match temp[row * len + col] {
                            Seat::Floor => continue,
                            Seat::Filled => {
                                count += 1;
                            }
                            Seat::Empty => {}
                        }
                        break;
                    }
                }

                // bottom
                {
                    let mut row = row;
                    // check top left
                    while row + 1 < height {
                        row += 1;
                        match temp[row * len + col] {
                            Seat::Floor => continue,
                            Seat::Filled => {
                                count += 1;
                            }
                            Seat::Empty => {}
                        }
                        break;
                    }
                }

                // bottom right
                {
                    let mut col = col;
                    let mut row = row;
                    // check top left
                    while row + 1 < height && col + 1 < len {
                        row += 1;
                        col += 1;
                        match temp[row * len + col] {
                            Seat::Floor => continue,
                            Seat::Filled => {
                                count += 1;
                            }
                            Seat::Empty => {}
                        }
                        break;
                    }
                }

                // do the swapping
                if matches!(temp[i], Seat::Filled) && count > 4 {
                    // dbg!(count, i);
                    swapped = true;
                    *ch = Seat::Empty;
                } else if matches!(temp[i], Seat::Empty) && count == 0 {
                    swapped = true;
                    *ch = Seat::Filled;
                } else {
                    *ch = temp[i].clone();
                }
            }
            display_seats(&vec, len);
        }

        display_seats(&vec, len);
        println!("seat that are filled: {}", count_filled(&vec));
    }
}
