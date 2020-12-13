use itertools::Itertools;
use std::fs::read_to_string;

fn gcd(a: u64, b: u64) -> i64 {
    let (x, y) = extended_euclidean_algorithm(a as i64, b as i64);

    x * a as i64 + y * b as i64
}

// This makes use of the fact that finding t such that
//
// t + a_0 is divisible by m_0 and
// t + a_1 is divisible by m_1 and ...
// t + a_2 is divisible by m_2 and ...
//
// can be expressed as congruence equations:
//
// t + a_0 ≡ 0 mod m_0
// t + a_1 ≡ 0 mod m_1
// ...
//
// which can be transformed to the equivalent:
//
// t ≡ a_0' mod m_0
// t ≡ a_1' mod m_1
// ...
//
// which can be solved for t using https://en.wikipedia.org/wiki/Chinese_remainder_theorem,
// assuming the m_i's are pairwise coprime.
fn part_2(time_table: &str) -> i64 {
    let bus_offset_tuples: Vec<(i64, i64)> = time_table
        .split(',')
        .enumerate()
        .filter(|(_, x)| *x != "x")
        // This results in congruence equations of the form
        // t + i ≡ 0 mod x
        .map(|(i, x)| (i as i64, x.parse().unwrap()))
        // Getting to normalized
        // t ≡ j mod x
        // with
        // j = (x - i) mod x
        // for Chinese remainder theorem
        .map(|(i, x)| ((x - i) % x, x))
        .collect();

    let moduli: Vec<i64> = bus_offset_tuples.iter().map(|x| x.1).collect();
    // The simple Chinese remainder theorem only works if all moduli are pairwise coprime.
    // Assert that's the case.
    assert!(
        moduli
            .iter()
            .permutations(2)
            .all(|rs| gcd(*rs[0] as u64, *rs[1] as u64) == 1),
        "not all remainders are pairwise coprime"
    );

    // See https://en.wikipedia.org/wiki/Chinese_remainder_theorem#Computation
    let remainder_product: i64 = moduli.iter().map(|x| *x as i64).product();
    let solution: i64 = bus_offset_tuples
        .iter()
        .map(|(a, m)| {
            let big_m = remainder_product / m;
            let (_r, s) = extended_euclidean_algorithm(*m, big_m);
            let e = s * big_m;
            a * e
        })
        .sum();

    let solution = solution % remainder_product;

    if solution < 0 {
        return remainder_product + solution;
    } else {
        return solution;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("7,13,x,x,59,x,31,19"), 1068781);
    }

    #[test]
    fn test_euclidean() {
        assert_eq!(extended_euclidean_algorithm(240, 46), (-9, 47));
    }
}

// See https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Pseudocode
fn extended_euclidean_algorithm(a: i64, b: i64) -> (i64, i64) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);
    while r != 0 {
        let quotient = old_r / r;

        // (old_r, r) := (r, old_r − quotient × r)
        std::mem::swap(&mut old_r, &mut r);
        r = r - old_r * quotient;

        // (old_s, s) := (s, old_s − quotient × s)
        std::mem::swap(&mut old_s, &mut s);
        s = s - old_s * quotient;

        // (old_t, t) := (t, old_t − quotient × t)
        std::mem::swap(&mut old_t, &mut t);
        t = t - old_t * quotient;
    }
    (old_s, old_t)
}

fn main() {
    let content = read_to_string("./inputs/day13.txt").expect("file not found");
    let mut lines = content.lines();
    let earliest_departure: i64 = lines.next().unwrap().parse().unwrap();
    let bus_ids: Vec<i64> = lines
        .next()
        .unwrap()
        .split(',')
        .filter(|x| *x != "x")
        .map(|x| x.parse().unwrap())
        .collect();

    let mut departure_time = earliest_departure;

    let solution;
    loop {
        let current_bus = bus_ids
            .iter()
            .find(|id| gcd(departure_time as u64, **id as u64) != 1);
        if current_bus.is_some() {
            let wait_time = departure_time - earliest_departure;
            solution = current_bus.unwrap() * wait_time;
            break;
        }

        departure_time += 1;
    }
    // Result: 3865
    assert_eq!(solution, 3865, "solution is no longer correct");
    println!("{:?}", solution);

    // Part 2:
    let solution = part_2(&content.lines().nth(1).unwrap());
    // Result: 415579909629976
    assert_eq!(solution, 415579909629976, "solution is no longer correct");
    println!("{}", solution);
}
