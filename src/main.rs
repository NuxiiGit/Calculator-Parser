mod parser;
mod input;

use parser::*;

fn main() {
    // initialise parser
    let mut parser : Parser<f64> = Parser::new();
    parser.add_op(Operator::new("(_)", 3, |args| args[0]));
    parser.add_op(Operator::new("|_|", 3, |args| if args[0] >= 0.0 {args[0]} else {-args[0]}));
    parser.add_op(Operator::new("_^_", 2, |args| args[0].powf(args[1])));
    parser.add_op(Operator::new("_*_", 1, |args| args[0] * args[1]));
    parser.add_op(Operator::new("_/_", 1, |args| args[0] / args[1]));
    parser.add_op(Operator::new("_+_", 0, |args| args[0] + args[1]));
    parser.add_op(Operator::new("_-_", 0, |args| args[0] - args[1]));
    // parse
    let mut expression = input::read_args_single(Some(1), None);
    loop {
        if let Some(tokens) = parser.parse(&expression) {
            if tokens.len() == 0 {
                println!("Please enter an expression: ");
                expression = input::read_buffer();
                continue; // messy "goto" start
            } else {
                if let Some(tree) = parser.build_token_tree(&tokens) {
                    if let Some(value) = tree.evaluate() {
                        println!("Value {}", value);
                    } else {
                        error("Unable to evaluate parse tree.");
                    }
                } else {
                    error("Unable to build parse tree.");
                }
            }
        } else {
            error("Unable to parse expression.");
        }
        break;
    }
}

fn error(msg : &str) {
    println!("Error: {}", msg);
}