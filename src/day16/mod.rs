use std::io::BufRead;

pub fn line_to_ticket(line: String) -> Vec<u64> {
    line.split(',').map(|a| a.parse::<u64>().unwrap()).collect()
}

pub fn run<'a>(reader: Box<dyn BufRead + 'a>) {
    let mut vec = vec![];
    let mut lines = reader.lines();
    let mut field_to_range: Vec<Vec<_>> = vec![];
    let mut points = vec![];
    let mut departure_index = vec![];

    // parse the fields
    loop {
        let line = lines.next().unwrap().unwrap();
        if line.trim().is_empty() {
            break;
        }
        let ranges = line.split(':').nth(1).unwrap().trim();
        for item in ranges.split(" or ") {
            let mut p = item.split('-').map(|c| c.parse::<u64>().unwrap());
            let range = (p.next().unwrap(), p.next().unwrap());
            vec.push(range);
            points.push(range);
        }
        departure_index.push(line.starts_with("departure"));
        field_to_range.push(points.drain(..).collect());
    }
    vec.sort_by_key(|a| a.0);

    // valid ranges combining all field ranges intersection
    let valid_numbers_ranges: Vec<_> = vec
        .into_iter()
        .fold(Vec::new(), |mut new_vec, a| {
            if let Some(p) = new_vec.pop() {
                let p: (u64, u64) = p;
                if a.0 <= p.1 as u64 {
                    new_vec.push((p.0, a.1.max(p.1)))
                } else {
                    new_vec.push(p);
                    new_vec.push(a)
                }
            } else {
                new_vec.push(a);
            }
            new_vec
        })
        .iter()
        .flat_map(|tup| std::iter::once(tup.0).chain(std::iter::once(tup.1)))
        .collect();

    lines.next().unwrap().unwrap(); // skip your ticket line
    let your_ticket = line_to_ticket(lines.next().unwrap().unwrap());

    let mut vec = vec![];
    for &content in your_ticket.iter() {
        let mut r = vec![];
        for (j, range) in field_to_range.iter().enumerate() {
            if range.iter().any(|&(a, b)| content >= a && content <= b) {
                r.push(j);
            }
        }
        vec.push(r);
    }

    lines.next().unwrap().unwrap(); // skip empty line
    lines.next().unwrap().unwrap(); // skip nearby ticket line

    let mut invalid_ticket_sum = 0;
    for ticket_line in lines {
        let mut valid_ticket = true;
        let ticket: Vec<_> = line_to_ticket(ticket_line.unwrap());
        ticket
            .iter()
            .for_each(|p| match valid_numbers_ranges.binary_search(&&p) {
                Err(n) if n % 2 == 0 => {
                    invalid_ticket_sum += p;
                    valid_ticket = false
                }
                _ => {}
            });
        if valid_ticket {
            ticket.iter().enumerate().for_each(|(i, p)| {
                vec.get_mut(i)
                    .unwrap()
                    .retain(|&r| field_to_range[r].iter().any(|&(a, b)| *p >= a && *p <= b));
            })
        }
    }

    println!("invalid ticket {}", invalid_ticket_sum);

    let mut field_to_ticket = vec![None; field_to_range.len()];

    while field_to_ticket.iter().any(|option| option.is_none()) {
        for (i, field_ids) in vec.iter_mut().enumerate() {
            if field_ids.len() == 1 {
                field_to_ticket[field_ids[0]] = Some(i);
            } else {
                field_ids.retain(|&i| field_to_ticket[i].is_none())
            }
        }
    }

    let product: u64 = departure_index
        .into_iter()
        .enumerate()
        .filter_map(|(i, b)| {
            if b {
                Some(your_ticket[field_to_ticket[i].unwrap()])
            } else {
                None
            }
        })
        .product();
    println!("multiply of all departures {}", product);
}
