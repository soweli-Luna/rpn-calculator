use std::{env, io::Read};

use parse::Token;

mod parse;
mod solve;

fn main() {

    for (i, arg) in env::args().enumerate().skip(1) {
        println!("Expression {i}:");

        // first try arg as filename
        if let Ok(mut file) = std::fs::File::open(&arg) {
            println!("Reading file `{}`:", arg);
            let mut buf = String::new();
            file.read_to_string(&mut buf).expect("failed to read file");

            parse_solve_and_print(buf);
        } else {
            // otherwise, try arg as an expression
            parse_solve_and_print(arg);
        }
        println!();
    }
}

fn parse_solve_and_print(expression: String) {
    let truncate_len = 100;

    let mut truncated_expr = expression.clone();
    truncated_expr.truncate(truncate_len);
    println!("`{}{}`", truncated_expr, if expression.len() > truncate_len { "..." } else { "" });

    let tokens = parse::Tokens::new(expression);
    let solve_result = solve::solve(tokens);

    match solve_result {
        Ok((stack, time)) => println!("Result: {:?}\n{time:?}", stack),
        Err(err) => eprintln!("solver error: {}", err),
    }
}
