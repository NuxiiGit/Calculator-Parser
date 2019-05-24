mod parser;
mod input;

use parser::*;

fn main() {
    let mut parser : Parser<f64> = Parser::new();
    // add operators of parser
    parser.add_op(Operator::new("(_)", 3, |args| args[0]));
    parser.add_op(Operator::new("|_|", 3, |args| {
        if args[0] >= 0.0 {
            args[0]
        } else {
            -args[0]
        }
    }));
    parser.add_op(Operator::new("_^_", 2, |args| args[0].powf(args[1])));
    parser.add_op(Operator::new("_*_", 1, |args| args[0] * args[1]));
    parser.add_op(Operator::new("_/_", 1, |args| args[0] / args[1]));
    parser.add_op(Operator::new("_+_", 0, |args| args[0] + args[1]));
    parser.add_op(Operator::new("_-_", 0, |args| args[0] - args[1]));
    // parse
    let expression : String = input::read_buffer();
    match parser.parse(&expression) {
        Some(tokens) => {
            if tokens.len() == 0 {
                println!("Error: Empty expression.");
            } else {
                match parser.build_token_tree(&tokens) {
                    Some(tree) => {
                        match tree.evaluate() {
                            Some(value) => println!("Value: {}", value),
                            None => println!("Error: Unable to evaluate parse tree.")
                        }
                    },
                    None => println!("Error: Unable to build parse tree.")
                }
            }
        },
        None => {
            println!("Error: Unable to parse expression.");
        }
    }
}