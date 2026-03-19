use crate::{env::Env, expression::Expression};
use color_eyre::Result;
use eyre::eyre;

pub fn eval_expression(expression: &Expression, env: &mut Env) -> Result<Expression> {
    match expression {
        Expression::Number(_)
        | Expression::String(_)
        | Expression::Function { params: _, body: _ }
        | Expression::Nil => Ok(expression.clone()),
        Expression::Symbol(s) => Ok(env.local.get(s).unwrap_or(&Expression::Nil).clone()),
        Expression::List(list) => {
            if list.is_empty() {
                Ok(Expression::Nil)
            } else if let Expression::Function { params, body } = eval_expression(&list[0], env)? {
                let mut function_env = env.clone();
                for i in 0..params.len() {
                    function_env
                        .local
                        .insert(params[i].as_symbol()?, eval_expression(&list[i + 1], env)?);
                }
                eval_expression(&body, &mut function_env)
            } else if let Expression::Symbol(symbol) = &list[0] {
                match symbol.as_str() {
                    "+" => eval_add(&list[1..], env),
                    "-" => eval_subtract(&list[1..], env),
                    "*" => eval_multiply(&list[1..], env),
                    "/" => eval_divide(&list[1..], env),
                    ">" => eval_greater(&list[1..], env),
                    "<" => eval_less(&list[1..], env),
                    "=" => eval_equal(&list[1..], env),
                    "and" => eval_and(&list[1..], env),
                    "or" => eval_or(&list[1..], env),
                    "not" => eval_not(&list[1..], env),
                    "quote" => eval_quote(&list[1..], env),
                    "function" => eval_function(&list[1..], env),
                    "define" => eval_define(&list[1..], env),
                    "if" => eval_if(&list[1..], env),
                    "map" => eval_map(&list[1..], env),
                    "file-read" => eval_file_read(&list[1..], env),
                    "split" => eval_split(&list[1..], env),
                    "as-number" => eval_as_number(&list[1..], env),
                    "inspect" => eval_inspect(&list[1..], env),
                    "apply" => eval_apply(&list[1..], env),
                    "quoted" => eval_quoted(&list[1..], env),
                    "fold" => eval_fold(&list[1..], env),
                    "trim" => eval_trim(&list[1..], env),
                    _ => Err(eyre!("Function '{}' doesn't exist", list[0])),
                }
            } else {
                Err(eyre!("Bomba {}", list[0]))
            }
        }
    }
}

fn eval_add(list: &[Expression], env: &mut Env) -> Result<Expression> {
    let evaluated_list = list
        .iter()
        .map(|e| eval_expression(e, env))
        .collect::<Result<Vec<Expression>>>()?;

    Ok(Expression::Number(
        evaluated_list
            .iter()
            .map(|e| e.as_number())
            .collect::<Result<Vec<isize>>>()?
            .into_iter()
            .reduce(|acc, e| acc + e)
            .ok_or(eyre!("No parameters provided"))?,
    ))
}

fn eval_subtract(list: &[Expression], env: &mut Env) -> Result<Expression> {
    let evaluated_list = list
        .iter()
        .map(|e| eval_expression(e, env))
        .collect::<Result<Vec<Expression>>>()?;

    Ok(Expression::Number(
        evaluated_list
            .iter()
            .map(|e| e.as_number())
            .collect::<Result<Vec<isize>>>()?
            .into_iter()
            .reduce(|acc, e| acc - e)
            .ok_or(eyre!("No parameters provided"))?,
    ))
}

fn eval_multiply(list: &[Expression], env: &mut Env) -> Result<Expression> {
    let evaluated_list = list
        .iter()
        .map(|e| eval_expression(e, env))
        .collect::<Result<Vec<Expression>>>()?;

    Ok(Expression::Number(
        evaluated_list
            .iter()
            .map(|e| e.as_number())
            .collect::<Result<Vec<isize>>>()?
            .into_iter()
            .reduce(|acc, e| acc * e)
            .ok_or(eyre!("No parameters provided"))?,
    ))
}

fn eval_divide(list: &[Expression], env: &mut Env) -> Result<Expression> {
    let evaluated_list = list
        .iter()
        .map(|e| eval_expression(e, env))
        .collect::<Result<Vec<Expression>>>()?;

    let evaluated_list = evaluated_list
        .iter()
        .map(|e| e.as_number())
        .collect::<Result<Vec<isize>>>()?;

    if evaluated_list.len() > 1 && evaluated_list[1..].contains(&0) {
        return Err(eyre!("Cannot divide by zero"));
    }

    Ok(Expression::Number(
        evaluated_list
            .into_iter()
            .reduce(|acc, e| acc / e)
            .ok_or(eyre!("No parameters provided"))?,
    ))
}

fn eval_greater(list: &[Expression], env: &mut Env) -> Result<Expression> {
    let left = eval_expression(&list[0], env)?;
    let right = eval_expression(&list[1], env)?;
    Ok((left.as_number()? > right.as_number()?).into())
}

fn eval_less(list: &[Expression], env: &mut Env) -> Result<Expression> {
    let left = eval_expression(&list[0], env)?;
    let right = eval_expression(&list[1], env)?;
    Ok((left.as_number()? < right.as_number()?).into())
}

fn eval_equal(list: &[Expression], env: &mut Env) -> Result<Expression> {
    let left = eval_expression(&list[0], env)?;
    let right = eval_expression(&list[1], env)?;
    Ok((left == right).into())
}

fn eval_and(list: &[Expression], env: &mut Env) -> Result<Expression> {
    let left = eval_expression(&list[0], env)?;
    let right = eval_expression(&list[1], env)?;

    if left.as_boolean()? {
        Ok(right)
    } else {
        Ok(Expression::Nil)
    }
}

fn eval_or(list: &[Expression], env: &mut Env) -> Result<Expression> {
    let left = eval_expression(&list[0], env)?;
    let right = eval_expression(&list[1], env)?;

    if left.as_boolean()? {
        Ok(left)
    } else {
        Ok(right)
    }
}

fn eval_not(list: &[Expression], env: &mut Env) -> Result<Expression> {
    let value = eval_expression(&list[0], env)?;
    Ok((!(value.as_boolean()?)).into())
}

fn eval_quote(list: &[Expression], _env: &mut Env) -> Result<Expression> {
    Ok(list[0].clone())
}

fn eval_function(list: &[Expression], env: &mut Env) -> Result<Expression> {
    let params = eval_expression(&list[0], env)?;
    let body = eval_expression(&list[1], env)?;

    Ok(Expression::Function {
        params: params.as_list()?,
        body: Box::new(body.clone()),
    })
}

fn eval_define(list: &[Expression], env: &mut Env) -> Result<Expression> {
    let key = eval_expression(&list[0], env)?.as_symbol()?;
    let value = eval_expression(&list[1], env)?;

    env.local.insert(key.clone(), value);

    Ok(Expression::Symbol(key))
}

fn eval_if(list: &[Expression], env: &mut Env) -> Result<Expression> {
    if list.len() < 2 {
        return Err(eyre!("Not enough parameters provided"));
    }
    let condition = eval_expression(&list[0], env)?;
    let true_result = eval_expression(&list[1], env);
    let false_result = if list.len() > 2 {
        eval_expression(&list[2], env)
    } else {
        Ok(Expression::Nil)
    };
    if condition.as_boolean()? {
        true_result
    } else {
        false_result
    }
}

fn eval_map(list: &[Expression], env: &mut Env) -> Result<Expression> {
    let function = eval_expression(&list[0], env)?;
    let values = eval_expression(&list[1], env)?;
    let mut results = vec![];

    for value in values.as_list()? {
        results.push(eval_expression(
            &Expression::List(vec![function.clone(), value]),
            env,
        )?);
    }
    Ok(Expression::List(results))
}

fn eval_file_read(list: &[Expression], env: &mut Env) -> Result<Expression> {
    let path = eval_expression(&list[0], env)?.as_string()?;
    let file = std::fs::read_to_string(path)?;
    Ok(Expression::String(file))
}

fn eval_split(list: &[Expression], env: &mut Env) -> Result<Expression> {
    let separator = eval_expression(&list[0], env)?.as_string()?;
    let string = eval_expression(&list[1], env)?.as_string()?;
    Ok(Expression::List(
        string
            .split(&separator)
            .map(|s| Expression::String(s.to_string()))
            .collect(),
    ))
}

fn eval_as_number(list: &[Expression], env: &mut Env) -> Result<Expression> {
    let string = eval_expression(&list[0], env)?.as_string()?;
    Ok(Expression::Number(string.parse().map_err(|_| {
        eyre!("Cannot parse '{}' as number", string)
    })?))
}

fn eval_inspect(list: &[Expression], env: &mut Env) -> Result<Expression> {
    let result = eval_expression(&list[0], env)?;
    println!("{}: {}", &list[0], result);
    Ok(result)
}

fn eval_apply(list: &[Expression], env: &mut Env) -> Result<Expression> {
    let function = eval_expression(&list[0], env)?;
    let values = eval_expression(&list[1], env)?.as_list()?;
    let mut transformed = vec![function];
    transformed.extend(values);
    eval_expression(&Expression::List(transformed), env)
}

fn eval_quoted(list: &[Expression], env: &mut Env) -> Result<Expression> {
    let expression = eval_expression(&list[0], env)?;
    Ok(Expression::List(vec![
        Expression::Symbol("quote".to_string()),
        expression,
    ]))
}

fn eval_fold(list: &[Expression], env: &mut Env) -> Result<Expression> {
    let function = eval_expression(&list[0], env)?;
    let initial = eval_expression(&list[1], env);
    let values = eval_expression(&list[2], env)?.as_list()?;

    values
        .into_iter()
        .fold(initial, |acc: Result<Expression>, e| {
            eval_expression(&Expression::List(vec![function.clone(), acc?, e]), env)
        })
}

fn eval_trim(list: &[Expression], env: &mut Env) -> Result<Expression> {
    let string = eval_expression(&list[0], env)?.as_string()?;
    Ok(Expression::String(string.trim().to_string()))
}
