use std::collections::HashMap;

#[derive(Debug)]
struct NumberStats {
    said_in_rounds: (usize, Option<usize>),
    said_count: usize,
}

fn main() {
    let content = "0,13,1,8,6,15";
    let start_numbers: Vec<u32> = content.split(",").map(|x| x.parse().unwrap()).collect();
    let mut number_stats: HashMap<u32, NumberStats> = HashMap::new();

    let mut current_round = 1;
    let mut last_number = 0;
    for x in start_numbers {
        number_stats.insert(
            x,
            NumberStats {
                said_in_rounds: (current_round, None),
                // NOTE: This assumes unique start numbers.
                said_count: 1,
            },
        );

        current_round += 1;
        last_number = x;
    }

    loop {
        //println!("turn: {}", current_round);
        let last_number_stats = number_stats
            .get(&last_number)
            // SAFETY: The `last_number` has already been added and is always accessible.
            .unwrap();
        //println!("looking at number: {}", last_number);
        //println!("said count: {}", last_number_stats.said_count);
        let new_number = if last_number_stats.said_count == 1 {
            //println!("first case");
            0
        } else {
            //println!("else case: {:?}", last_number_stats.said_in_rounds);
            (last_number_stats.said_in_rounds.0 - last_number_stats.said_in_rounds.1.unwrap())
                as u32
        };
        //println!("new number: {}", new_number);

        number_stats
            .entry(new_number)
            .and_modify(|stats| {
                stats.said_count += 1;
                stats.said_in_rounds = (current_round, Some(stats.said_in_rounds.0));
            })
            .or_insert(NumberStats {
                said_in_rounds: (current_round, None),
                said_count: 1,
            });
        // Result:
        // spoken in round 2020: 1618
        // spoken in round 30000000: 548531
        if current_round == 2020 {
            println!("spoken in round 2020: {}", new_number);
        }
        if current_round == 30000000 {
            println!("spoken in round 30000000: {}", new_number);
            break;
        }
        current_round += 1;
        last_number = new_number;
    }
}
