use crate::expression::Expression;
use core::panic;
use std::{iter::Peekable, str::Chars};

fn parse_number(chars: &mut Peekable<Chars>) -> Expression {
    let mut new_str = String::new();

    while let Some(c) = chars.peek() {
        if !c.is_numeric() {
            break;
        }
        new_str.push(*c);
        chars.next();
    }

    Expression::Number(new_str.parse().unwrap())
}

fn parse_symbol(chars: &mut Peekable<Chars>) -> Expression {
    let mut symbol = String::new();
    while let Some(c) = chars.peek() {
        if c.is_whitespace() || *c == ')' {
            break;
        }
        symbol.push(*c);
        chars.next();
    }

    Expression::Symbol(symbol)
}

fn parse_string(chars: &mut Peekable<Chars>) -> Expression {
    let mut new_str = String::new();
    chars.next();

    while let Some(&c) = &chars.peek() {
        if c == '"' {
            chars.next();
            break;
        }
        if c == '\\' {
            chars.next();
            if let Some(c) = chars.peek() {
                match c {
                    'n' => new_str.push('\n'),
                    'r' => new_str.push('\r'),
                    't' => new_str.push('\t'),
                    '\\' => new_str.push('\\'),
                    '"' => new_str.push('"'),
                    _ => panic!("Unknown escape character"),
                }
                chars.next();
                continue;
            } else {
                panic!("Unexpected end of string");
            }
        }
        new_str.push(c);
        chars.next();
    }

    Expression::String(new_str)
}

fn parse_list(chars: &mut Peekable<Chars>) -> Expression {
    let mut new_list = vec![];
    chars.next();

    while let Some(c) = chars.peek() {
        if *c == ')' {
            chars.next();
            break;
        }
        if c.is_whitespace() {
            chars.next();
            continue;
        }
        new_list.push(parse_expression(chars));
    }

    Expression::List(new_list)
}

fn parse_quote(chars: &mut Peekable<Chars>) -> Expression {
    chars.next();
    Expression::List(vec![
        Expression::Symbol("quote".to_string()),
        parse_expression(chars),
    ])
}

fn parse_inspect(chars: &mut Peekable<Chars>) -> Expression {
    chars.next();
    Expression::List(vec![
        Expression::Symbol("inspect".to_string()),
        parse_expression(chars),
    ])
}

pub fn parse_expression(chars: &mut Peekable<Chars>) -> Expression {
    loop {
        let c = chars.peek().unwrap();
        if c.is_whitespace() {
            chars.next();
            continue;
        }
        return if c.is_numeric() {
            parse_number(chars)
        } else if *c == '"' {
            parse_string(chars)
        } else if *c == '(' {
            parse_list(chars)
        } else if *c == '\'' {
            parse_quote(chars)
        } else if *c == '!' {
            parse_inspect(chars)
        } else {
            parse_symbol(chars)
        };
    }
}
