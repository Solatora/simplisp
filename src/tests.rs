use crate::{env::Env, eval::eval_expression, expression::Expression, parse::parse_expression};

fn parse(input: &str) -> Expression {
    parse_expression(&mut input.chars().peekable())
}

#[test]
fn parse_number() {
    assert_eq!(parse("123"), Expression::Number(123));
}

#[test]
fn parse_symbol() {
    assert_eq!(parse("hello"), Expression::Symbol("hello".to_string()));
}

#[test]
fn parse_string() {
    assert_eq!(parse("\"hello\""), Expression::String("hello".to_string()));
}

#[test]
fn parse_list() {
    assert_eq!(
        parse("(hello 123)"),
        Expression::List(vec![
            Expression::Symbol("hello".to_string()),
            Expression::Number(123)
        ])
    );
}

#[test]
fn eval_nil() {
    let mut env = Env::std_lib();
    assert_eq!(
        eval_expression(&parse("()"), &mut env).unwrap(),
        Expression::Nil
    );
}

#[test]
fn eval_add() {
    let mut env = Env::std_lib();
    assert_eq!(
        eval_expression(&parse("(+ 1 2)"), &mut env).unwrap(),
        Expression::Number(3)
    );
}

#[test]
fn eval_sub() {
    let mut env = Env::std_lib();
    assert_eq!(
        eval_expression(&parse("(- 1 2)"), &mut env).unwrap(),
        Expression::Number(-1)
    );
}

#[test]
fn eval_mul() {
    let mut env = Env::std_lib();
    assert_eq!(
        eval_expression(&parse("(* 1 2)"), &mut env).unwrap(),
        Expression::Number(2)
    );
}

#[test]
fn eval_div() {
    let mut env = Env::std_lib();
    assert_eq!(
        eval_expression(&parse("(/ 1 2)"), &mut env).unwrap(),
        Expression::Number(0)
    );
}

#[test]
fn eval_if() {
    let mut env = Env::std_lib();
    assert_eq!(
        eval_expression(&parse("(if true true)"), &mut env).unwrap(),
        Expression::Symbol("t".to_string())
    );
    assert_eq!(
        eval_expression(&parse("(if false true false)"), &mut env).unwrap(),
        Expression::Nil
    );
    assert_eq!(
        eval_expression(&parse("(if false true)"), &mut env).unwrap(),
        Expression::Nil
    );
}

#[test]
fn eval_quote() {
    let mut env = Env::std_lib();
    assert_eq!(
        eval_expression(&parse("(quote (+ 1 2))"), &mut env).unwrap(),
        Expression::List(vec![
            Expression::Symbol("+".to_string()),
            Expression::Number(1),
            Expression::Number(2)
        ])
    );
    assert_eq!(
        eval_expression(&parse("'x"), &mut env).unwrap(),
        Expression::Symbol('x'.to_string())
    );
}

#[test]
fn eval_define() {
    let mut env = Env::std_lib();
    eval_expression(&parse("(define 'x 10)"), &mut env).unwrap();
    assert_eq!(
        eval_expression(&parse("x"), &mut env).unwrap(),
        Expression::Number(10)
    );
}
