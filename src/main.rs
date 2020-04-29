use std::io;
use stringed::{eval_expr, parse};

fn main() {
    println!("Stringed REPL");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line D:");
        let expr = match parse(&input.trim()) {
            Ok(expr) => expr,
            Err(err) => {
                println!("error: {}", err);
                continue;
            }
        };
        if expr.have_input {
            println!("accepting inputs...");
            loop {
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line D:");
                match eval_expr(&expr.expr, &input.trim()) {
                    Ok(result) => println!("= {}", result),
                    Err(err) => println!("error: {}", err),
                }
            }
        } else {
            match eval_expr(&expr.expr, "") {
                Ok(result) => println!("= {}", result),
                Err(err) => println!("error: {}", err),
            }
        }
    }
}
