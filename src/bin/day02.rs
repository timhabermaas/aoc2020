use core::char::ParseCharError;
use core::num::ParseIntError;
use core::str::FromStr;
use regex::Regex;
use std::fs::read_to_string;

#[derive(Debug)]
struct Line {
    min: usize,
    max: usize,
    character: char,
    password: String,
}

#[derive(Debug)]
struct LineParseError {}

impl From<ParseIntError> for LineParseError {
    fn from(_err: ParseIntError) -> Self {
        LineParseError {}
    }
}

impl From<ParseCharError> for LineParseError {
    fn from(_err: ParseCharError) -> Self {
        LineParseError {}
    }
}

impl FromStr for Line {
    type Err = LineParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<c>[a-z]): (?P<pw>[a-z]+)").unwrap();
        let captures = regex.captures(s).unwrap();

        let min = captures
            .name("min")
            .ok_or(LineParseError {})?
            .as_str()
            .parse()?;
        let max = captures
            .name("max")
            .ok_or(LineParseError {})?
            .as_str()
            .parse()?;
        let character = captures
            .name("c")
            .ok_or(LineParseError {})?
            .as_str()
            .parse()?;
        let password = captures
            .name("pw")
            .ok_or(LineParseError {})?
            .as_str()
            .to_owned();

        Ok(Line {
            min,
            max,
            character,
            password,
        })
    }
}

fn contains_valid_password_1(line: &Line) -> bool {
    let char_count = line
        .password
        .chars()
        .filter(|c| *c == line.character)
        .count();

    return line.min <= char_count && char_count <= line.max;
}

fn contains_valid_password_2(line: &Line) -> bool {
    let first = line.password.chars().nth(line.min - 1).unwrap();
    let second = line.password.chars().nth(line.max - 1).unwrap();

    return first == line.character && second != line.character
        || first != line.character && second == line.character;
}

fn main() {
    let line_iter = read_to_string("./inputs/day02.txt").expect("file not found");
    let lines = line_iter
        .lines()
        .map(|x| x.parse::<Line>().expect("not a valid Line"));

    let correct_pw_count_1 = lines.clone().filter(contains_valid_password_1).count();
    // Result: 603
    println!("{:?}", correct_pw_count_1);

    let correct_pw_count_2 = lines.filter(contains_valid_password_2).count();
    // Result: 404
    println!("{:?}", correct_pw_count_2);
}
