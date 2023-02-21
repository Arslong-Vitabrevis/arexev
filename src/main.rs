use std::io;

mod parsemath;
use parsemath::ast;
use parsemath::parser::{Parser, ParserError};

fn main() {
    println!("Hello! Welcome to Aritmetic expression evaluator. \nYou can calculate value for expression such as 2*3+(4-5)+2^3/4. \nAllowed numbers: positive, negative and decimals. \nSupported operations: Add,Subtract,Multiply,Divide,PowerOf(^). ");
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match evaluate(input) {
                    Ok(val) => println!("The computed number is {}\n", val),
                    Err(_) => {
                        println!("Error in evaluating expression. Please enter valid expression \n")
                    }
                };
            }
            Err(error) => println!("Error {}", error),
        }
    }
}

fn evaluate(expr: String) -> Result<f64, ParserError> {
    let expr = expr.split_whitespace().collect::<String>();
    let mut math_parser = Parser::new(&expr)?;
    let ast = math_parser.parse()?;
    println!("The generate AST is {:?}", ast);
    Ok(ast::eval(ast).unwrap())
}
