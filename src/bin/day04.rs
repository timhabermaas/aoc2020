use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug, PartialEq)]
enum Distance {
    Cm(i32),
    In(i32),
}
use Distance::*;

fn parse_height(height: &str) -> Option<Distance> {
    let reg = Regex::new(r"(\d+)(cm|in)").unwrap();
    match reg.captures(height) {
        Some(cap) => match cap.get(2).unwrap().as_str() {
            "cm" => Some(Cm(cap.get(1).unwrap().as_str().parse::<i32>().unwrap())),
            _ => Some(In(cap.get(1).unwrap().as_str().parse::<i32>().unwrap())),
        },
        None => None,
    }
}

fn parse_haircolor(color: &str) -> Option<()> {
    let reg = Regex::new(r"#[0-9a-f]{6}").unwrap();
    if reg.is_match(color) {
        Some(())
    } else {
        None
    }
}

fn parse_eyecolor(color: &str) -> Option<()> {
    let reg = Regex::new(r"(amb|blu|brn|gry|grn|hzl|oth)").unwrap();
    if reg.is_match(color) {
        Some(())
    } else {
        None
    }
}

fn parse_pid(pid: &str) -> Option<()> {
    let reg = Regex::new(r"\d{9}").unwrap();
    if reg.is_match(pid) {
        Some(())
    } else {
        None
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_parse_height() {
        assert_eq!(parse_height("134cm"), Some(Cm(134)));
        assert_eq!(parse_height("134m"), None);
        assert_eq!(parse_height("23in"), Some(In(23)));
        assert_eq!(parse_height("23n"), None);
    }

    fn test_parse_haircolor() {
        assert_eq!(parse_haircolor("#123abc"), Some(()));
        assert_eq!(parse_haircolor("#123abz"), None);
        assert_eq!(parse_haircolor("a#123abc"), None);
    }
}

fn is_valid(key: &str, value: Option<&str>) -> bool {
    match key {
        "byr" => value
            .filter(|x| {
                x.parse::<i32>()
                    .ok()
                    .filter(|x| *x >= 1920 && *x <= 2002)
                    .is_some()
            })
            .is_some(),
        "iyr" => value
            .filter(|x| {
                x.parse::<i32>()
                    .ok()
                    .filter(|x| *x >= 2010 && *x <= 2020)
                    .is_some()
            })
            .is_some(),
        "eyr" => value
            .filter(|x| {
                x.parse::<i32>()
                    .ok()
                    .filter(|x| *x >= 2020 && *x <= 2030)
                    .is_some()
            })
            .is_some(),
        "hgt" => value
            .and_then(parse_height)
            .filter(|h| match h {
                Cm(x) => *x >= 150 && *x <= 193,
                In(x) => *x >= 59 && *x <= 76,
            })
            .is_some(),
        "hcl" => value.and_then(parse_haircolor).is_some(),
        "ecl" => value.and_then(parse_eyecolor).is_some(),
        "pid" => value.and_then(parse_pid).is_some(),
        _ => true,
    }
}

fn main() {
    let required_keys: HashSet<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"]
        .iter()
        .map(|x| x.to_owned())
        .collect();
    let content = read_to_string("./inputs/day04.txt").expect("file not found");

    let newline_or_space = Regex::new(r"\n| ").unwrap();
    let passport_keys = content.split("\n\n").map(|passport_data| {
        newline_or_space
            .split(passport_data)
            .map(|kv| {
                let mut it = kv.split(':');
                (it.next().unwrap(), it.next().unwrap())
            })
            .collect::<HashMap<_, _>>()
    });

    let mut valid_count = 0;

    // Problem 1
    for kvs in passport_keys.clone() {
        let keys = kvs.keys().map(|x| x.to_owned()).collect::<HashSet<_>>();
        let diff = required_keys
            .difference(&keys)
            .map(|x| x.to_owned())
            .collect::<Vec<_>>();
        if diff == vec!["cid"] || diff.is_empty() {
            valid_count += 1;
        }
    }
    // Result: 239
    println!("{:?}", valid_count);

    let mut valid_count = 0;
    for map in passport_keys.clone() {
        let is_valid = is_valid("byr", map.get("byr").map(|x| *x))
            && is_valid("byr", map.get("byr").map(|x| *x))
            && is_valid("iyr", map.get("iyr").map(|x| *x))
            && is_valid("eyr", map.get("eyr").map(|x| *x))
            && is_valid("hgt", map.get("hgt").map(|x| *x))
            && is_valid("hcl", map.get("hcl").map(|x| *x))
            && is_valid("ecl", map.get("ecl").map(|x| *x))
            && is_valid("pid", map.get("pid").map(|x| *x))
            && is_valid("cid", map.get("cid").map(|x| *x));

        if is_valid {
            println!("{:?}", map);
            valid_count += 1;
        }
    }
    println!("{:?}", valid_count);
}
