use std::io::BufRead;

#[inline(never)]
pub fn compute_permutation(len: usize, previous_zero: bool) -> usize {
    if len == 0 {
        1
    } else if len == 1 {
        if previous_zero {
            2
        } else {
            2
        }
    } else {
        if previous_zero {
            compute_permutation(len - 1, false) + compute_permutation(len - 2, false)
        } else {
            compute_permutation(len - 1, true) + compute_permutation(len - 1, false)
        }
    }
}

pub fn run<'a>(reader: Box<dyn BufRead + 'a>) {
    let mut vec = vec![];
    let mut compulsory = vec![];
    for input in reader.lines() {
        let line = input.unwrap();
        vec.push(line.trim().parse::<usize>().unwrap());
    }
    // the changer outlet is 0
    vec.push(0);
    vec.sort();
    vec.push(vec.last().unwrap() + 3);

    let jolt_diff = vec
        .windows(2)
        .enumerate()
        .fold(
            ([0, 0, 0], false),
            |(mut sum, last_was_one_diff), (i, p)| {
                let diff = p[1] - p[0] - 1;
                sum[diff] += 1;
                if diff == 0 && !last_was_one_diff {
                    compulsory.push(i + 1);
                }
                if diff == 2 && last_was_one_diff {
                    compulsory.push(i);
                }
                (sum, diff == 0)
            },
        )
        .0;

    let arrangements = compulsory.chunks_exact(2).fold(1, |product, p| {
        let p = vec[p[0]..p[1]].as_ref();
        product
            * if p.is_empty() {
                1
            } else {
                compute_permutation(p.len(), false)
            }
    });

    println!("jolt 3 x 1 is {}", jolt_diff[0] * jolt_diff[2]);
    println!("arrangement are {}", arrangements);
}
