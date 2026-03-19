use color_eyre::Result;
use expression::Expression;
use std::io::Write;

#[cfg(test)]
mod tests;

mod env;
use env::Env;

mod expression;
// use expression::Expression;

mod parse;

use parse::parse_expression;

mod eval;
use eval::eval_expression;

fn get_input(msg: &str) -> String {
    let mut input = String::new();

    print!("{msg}");

    std::io::stdout().flush().unwrap();

    std::io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

fn main() -> Result<()> {
    // color_eyre::install()?;
    let mut env = Env::std_lib();
    let file_name = std::env::args().nth(1);

    if let Some(file_name) = file_name {
        let lisp = std::fs::read_to_string(file_name)?;
        let mut chars = lisp.trim().chars().peekable();
        let mut expression: Result<Expression> = Ok(Expression::Nil);

        while chars.size_hint().1.unwrap() > 0 {
            expression = Ok(eval_expression(&parse_expression(&mut chars), &mut env)?);
        }

        match expression {
            Ok(e) => println!("{e}"),
            Err(err) => println!("{:?}", err),
        }

        Ok(())
    } else {
        loop {
            let lisp = get_input("> ");

            if lisp.trim() == "" {
                continue;
            }

            let expression = parse_expression(&mut lisp.chars().peekable());

            let result = eval_expression(&expression, &mut env);

            match result {
                Ok(e) => println!("{e}"),
                Err(err) => println!("{:?}", err),
            }
        }
    }

    // Ok(())
}
