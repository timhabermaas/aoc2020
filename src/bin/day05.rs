use std::fs::read_to_string;

fn bits_to_byte(bits: &[bool]) -> u8 {
    let foo = bits.iter().rev().fold((0, 0), |(acc, ex), x| {
        (acc + (*x as u8) * (2 as u8).pow(ex), ex + 1)
    });
    foo.0
}

fn main() {
    let input = read_to_string("./inputs/day05.txt").expect("file not found");
    let lines = input.lines();

    let seat_ids = lines.map(|line| {
        let first_part = line[0..7].chars().map(|x| match x {
            'F' => false,
            'B' => true,
            _ => false,
        });

        let row = bits_to_byte(first_part.collect::<Vec<_>>().as_slice());

        let second_part = line[7..10].chars().map(|x| match x {
            'L' => false,
            'R' => true,
            _ => false,
        });
        let column = bits_to_byte(second_part.collect::<Vec<_>>().as_slice());
        let seat_id = row as u32 * 8 + column as u32;
        seat_id
    });

    let mut seats = seat_ids.collect::<Vec<_>>();
    seats.sort();

    // Part 1
    // Result: 892
    println!("{:?}", seats.iter().last());

    // Part 2
    let missing_seat_id = seats.iter().zip(seats.iter().skip(1)).find_map(|(x, y)| {
        if *y as i32 - *x as i32 > 1 {
            Some(x + 1)
        } else {
            None
        }
    });
    // Result: 625
    println!("{:?}", missing_seat_id);
}
