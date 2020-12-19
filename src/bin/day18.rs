use std::fs::read_to_string;
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
enum Expression {
    Binary(Operator, Rc<Expression>, Rc<Expression>),
    Literal(i64),
    Grouped(Rc<Expression>),
}

#[derive(Debug, PartialEq, Clone)]
enum Operator {
    Mul,
    Add,
}

use Expression::*;

#[derive(Debug, PartialEq)]
enum ParseError {
    GenericParseError(String),
}

fn try_parse_literal(lit: &str, part_1: bool) -> Result<(Expression, &str), ParseError> {
    match lit.chars().nth(0).unwrap() {
        '(' => {
            let (expr, rest) = if part_1 {
                parse_inner_expression(&lit[1..])?
            } else {
                try_parse_product(&lit[1..])?
            };
            if rest.chars().nth(0) != Some(')') {
                return Err(ParseError::GenericParseError(
                    "missing closing )".to_owned(),
                ));
            }
            return Ok((Grouped(Rc::new(expr)), &rest[1..]));
        }
        '0'..='9' => {
            let digit_chars = lit.chars().take_while(|c| c.is_digit(10));
            return Ok((
                Literal(digit_chars.clone().collect::<String>().parse().unwrap()),
                &lit[digit_chars.count()..],
            ));
        }
        _ => Err(ParseError::GenericParseError(
            "neither number nor (".to_owned(),
        )),
    }
}

fn try_parse_operator(
    op: &str,
    c: char,
    operator: Operator,
) -> Result<(Operator, &str), ParseError> {
    if op.chars().nth(0) == Some(c) {
        Ok((operator, &op[1..]))
    } else {
        Err(ParseError::GenericParseError(format!("no {}", c)))
    }
}

fn try_parse_some_operator(op: &str) -> Result<(Operator, &str), ParseError> {
    return try_parse_operator(&op, '*', Operator::Mul)
        .or(try_parse_operator(&op, '+', Operator::Add))
        .map_err(|_| ParseError::GenericParseError("neither + nor *".to_owned()));
}

fn try_parse_product(expr: &str) -> Result<(Expression, &str), ParseError> {
    let (lit1, rest) = try_parse_sum(&expr)?;
    let mut rest_expr = vec![];
    let mut rest = rest;
    loop {
        if let Ok((op, after_operator)) = try_parse_operator(&rest, '*', Operator::Mul) {
            let (lit2, after_literal) = try_parse_sum(&after_operator)?;
            rest_expr.push((op, lit2));
            rest = after_literal;
        } else {
            break;
        }
    }
    if rest_expr.is_empty() {
        return Ok((lit1, rest));
    }
    let mut left_bottom_tree = Binary(
        rest_expr[0].0.clone(),
        Rc::new(lit1),
        Rc::new(rest_expr[0].1.clone()),
    );

    // TODO: Use detain from vec to move out of vec without `clone`.
    for (op, exp) in &rest_expr[1..] {
        left_bottom_tree = Binary(op.clone(), Rc::new(left_bottom_tree), Rc::new(exp.clone()));
    }

    Ok((left_bottom_tree, rest))
}

fn try_parse_sum(expr: &str) -> Result<(Expression, &str), ParseError> {
    let (lit1, rest) = try_parse_literal(&expr, false)?;
    let mut rest_expr = vec![];
    let mut rest = rest;
    loop {
        if let Ok((op, after_operator)) = try_parse_operator(&rest, '+', Operator::Add) {
            let (lit2, after_literal) = try_parse_literal(&after_operator, false)?;
            rest_expr.push((op, lit2));
            rest = after_literal;
        } else {
            break;
        }
    }
    if rest_expr.is_empty() {
        return Ok((lit1, rest));
    }
    let mut left_bottom_tree = Binary(
        rest_expr[0].0.clone(),
        Rc::new(lit1),
        Rc::new(rest_expr[0].1.clone()),
    );

    // TODO: Use detain from vec to move out of vec without `clone`.
    for (op, exp) in &rest_expr[1..] {
        left_bottom_tree = Binary(op.clone(), Rc::new(left_bottom_tree), Rc::new(exp.clone()));
    }

    Ok((left_bottom_tree, rest))
}

fn parse_inner_expression(expr: &str) -> Result<(Expression, &str), ParseError> {
    let (lit1, rest) = try_parse_literal(&expr, true)?;
    let mut rest_expr = vec![];
    let mut rest = rest;
    loop {
        if let Ok((op, after_operator)) = try_parse_some_operator(&rest) {
            let (lit2, after_literal) = try_parse_literal(&after_operator, true)?;
            rest_expr.push((op, lit2));
            rest = after_literal;
        } else {
            break;
        }
    }
    if rest_expr.is_empty() {
        return Ok((lit1, rest));
    }
    let mut left_bottom_tree = Binary(
        rest_expr[0].0.clone(),
        Rc::new(lit1),
        Rc::new(rest_expr[0].1.clone()),
    );

    // TODO: Use detain from vec to move out of vec without `clone`.
    for (op, exp) in &rest_expr[1..] {
        left_bottom_tree = Binary(op.clone(), Rc::new(left_bottom_tree), Rc::new(exp.clone()));
    }

    Ok((left_bottom_tree, rest))
}

fn parse_expression_part_1(expr: &str) -> Result<Expression, ParseError> {
    parse_inner_expression(&expr).map(|(e, _r)| e)
}

fn parse_expression_part_2(expr: &str) -> Result<Expression, ParseError> {
    try_parse_product(&expr).map(|(e, _r)| e)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_expression_part_1() {
        assert_eq!(
            parse_expression_part_1("((2+3)*4)"),
            Ok(Expression::Grouped(Rc::new(Expression::Binary(
                Operator::Mul,
                Rc::new(Expression::Grouped(Rc::new(Expression::Binary(
                    Operator::Add,
                    Rc::new(Expression::Literal(2)),
                    Rc::new(Expression::Literal(3))
                )))),
                Rc::new(Expression::Literal(4))
            ))))
        );
        assert_eq!(
            parse_expression_part_1("2+3*4"),
            Ok(Expression::Binary(
                Operator::Mul,
                Rc::new(Expression::Binary(
                    Operator::Add,
                    Rc::new(Expression::Literal(2)),
                    Rc::new(Expression::Literal(3))
                )),
                Rc::new(Expression::Literal(4)),
            ))
        );

        assert_eq!(
            parse_expression_part_1("2+(3*4)+3"),
            Ok(Expression::Binary(
                Operator::Add,
                Rc::new(Expression::Binary(
                    Operator::Add,
                    Rc::new(Expression::Literal(2)),
                    Rc::new(Expression::Grouped(Rc::new(Expression::Binary(
                        Operator::Mul,
                        Rc::new(Expression::Literal(3)),
                        Rc::new(Expression::Literal(4))
                    ))))
                )),
                Rc::new(Expression::Literal(3)),
            ))
        );
    }

    #[test]
    fn test_parse_expression_part_2() {
        assert_eq!(
            parse_expression_part_2("2*3+4"),
            Ok(Expression::Binary(
                Operator::Mul,
                Rc::new(Expression::Literal(2)),
                Rc::new(Expression::Binary(
                    Operator::Add,
                    Rc::new(Expression::Literal(3)),
                    Rc::new(Expression::Literal(4))
                ))
            ))
        );
    }

    #[test]
    fn examples() {
        let exp = parse_expression_part_2("1+(2*3)+(4*(5+6))").expect("correct parsing");
        assert_eq!(interpret(&exp), 51);

        let exp = parse_expression_part_2("2*3+(4*5)").expect("correct parsing");
        assert_eq!(interpret(&exp), 46);

        let exp = parse_expression_part_2("((2+4*9)*(6+9*8+6)+6)+2+4*2").expect("correct parsing");
        assert_eq!(interpret(&exp), 23340);
    }
}

fn remove_whitespace(str: &str) -> String {
    str.chars().filter(|c| !c.is_whitespace()).collect()
}

fn interpret(expr: &Expression) -> i64 {
    match expr {
        Grouped(expr) => interpret(expr),
        Binary(Operator::Mul, e1, e2) => interpret(e1) * interpret(e2),
        Binary(Operator::Add, e1, e2) => interpret(e1) + interpret(e2),
        Literal(l) => *l,
    }
}

fn main() {
    let content = read_to_string("./inputs/day18.txt").expect("file not found");

    let lines = content.lines().map(remove_whitespace);

    // Part 1
    let result_1: i64 = lines
        .clone()
        .map(|l| parse_expression_part_1(&l))
        .map(Result::unwrap)
        .map(|x| interpret(&x))
        .sum();

    // Result: 98621258158412
    println!("{:?}", result_1);

    // Part 2
    let result_2: i64 = lines
        .clone()
        .map(|l| parse_expression_part_2(&l))
        .map(Result::unwrap)
        .map(|e| interpret(&e))
        .sum();

    // Result: 241216538527890
    println!("{:?}", result_2);
}
