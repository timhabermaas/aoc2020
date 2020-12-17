use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

fn is_in_range(number: u32, ((a1, b1), (a2, b2)): &((u32, u32), (u32, u32))) -> bool {
    number >= *a1 && number <= *b1 || number >= *a2 && number <= *b2
}

fn parse_range(range: &str) -> (u32, u32) {
    let mut numbers = range.split('-');
    (
        numbers.next().unwrap().parse().unwrap(),
        numbers.next().unwrap().parse().unwrap(),
    )
}

fn parse_number_list(list: &str) -> Vec<u32> {
    list.split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn limit_possible_fields(number: u32, conditions: &HashSet<Field>) -> HashSet<Field> {
    conditions
        .iter()
        .filter(|field| is_in_range(number, &field.conditions))
        .cloned()
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Field {
    name: String,
    conditions: ((u32, u32), (u32, u32)),
}

fn main() {
    let content = read_to_string("./inputs/day16.txt").expect("file not found");

    let parts = content.split("\n\n").collect::<Vec<_>>();
    let fields: HashSet<Field> = parts[0]
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let name = parts.next().unwrap();
            let mut ranges = parts.next().unwrap().split(" or ");
            let first = ranges.next().unwrap();
            let second = ranges.next().unwrap();

            Field {
                name: name.to_owned(),
                conditions: (parse_range(first), parse_range(second)),
            }
        })
        .collect();
    let your_ticket = parse_number_list(parts[1].lines().nth(1).unwrap());
    let nearby_tickets: Vec<Vec<u32>> = parts[2]
        .lines()
        .skip(1)
        .map(parse_number_list)
        .collect::<Vec<_>>();

    // Part 1
    // We want to find the tickets for which there are is least one field which doesn't fit into
    // any field condition.
    let mut invalid_fields: Vec<u32> = Vec::new();
    for nearby_ticket in nearby_tickets.clone() {
        for ticket_field in nearby_ticket {
            let remaining_possible_fields = limit_possible_fields(ticket_field, &fields);
            if remaining_possible_fields.is_empty() {
                invalid_fields.push(ticket_field);
                break;
            }
        }
    }
    // Result: 20975
    println!("{:?}", invalid_fields.iter().sum::<u32>());

    // Part 2
    let possible_fields: Vec<HashSet<Field>> = fields
        .iter()
        .map(|_| fields.iter().cloned().collect())
        .collect();

    let mut possible_fields_per_row: Vec<Vec<HashSet<Field>>> = Vec::new();

    for nearby_ticket in nearby_tickets.clone() {
        let mut possible_field_per_row = possible_fields.clone();
        for (i, field) in nearby_ticket.iter().enumerate() {
            possible_field_per_row[i] = limit_possible_fields(*field, &possible_field_per_row[i]);
        }
        possible_fields_per_row.push(possible_field_per_row);
    }

    // Filter invalid tickets. Invalid tickets are tickets which have at least one unfulfillable
    // category.
    let all_valid_tickets = possible_fields_per_row
        .iter()
        .filter(|x| x.iter().all(|x| x.len() != 0))
        .collect::<Vec<_>>();

    let mut possible_fields_per_column = Vec::new();
    for i in 0..all_valid_tickets[0].len() {
        let mut possible_fields: Option<HashSet<Field>> = None;
        for j in 0..all_valid_tickets.len() {
            possible_fields = possible_fields
                .map(|x| x.intersection(&all_valid_tickets[j][i]).cloned().collect())
                .or(Some(all_valid_tickets[j][i].clone()));
        }
        println!("fields for column {}: {:?}", i, possible_fields);
        possible_fields_per_column.push(possible_fields.unwrap());
    }
    // Maps from an index to the field it belongs to.
    let mut solution_map: HashMap<usize, Field> = HashMap::new();

    loop {
        // Loop until we no longer find a set with exactly one solution.
        if let Some((min_column, min_field_set)) = possible_fields_per_column
            .iter()
            .enumerate()
            .find(|(_i, x)| x.len() == 1)
        {
            let min_field = min_field_set.iter().next().unwrap().clone();

            for set in possible_fields_per_column.iter_mut() {
                // Remove found category from every field set.
                set.remove(&min_field);
            }
            assert_eq!(solution_map.insert(min_column, min_field), None);
        } else {
            break;
        }
    }
    println!("{:#?}", solution_map);
    let departure_indices = solution_map
        .iter()
        .filter(|(_index, field)| field.name.starts_with("departure"))
        .map(|(index, _field)| index)
        .collect::<Vec<_>>();

    let solution: u64 = your_ticket
        .iter()
        .enumerate()
        .filter(|(i, _x)| departure_indices.contains(&i))
        .map(|(_i, x)| *x as u64)
        .product();

    // Result: 910339449193
    println!("{:#?}", solution);
}
