mod parser;
mod input;

use parser::*;

fn main() {
    let args : Vec<String> = input::read_args(Some(1), None);
    if args.len() != 2 {
        println!("Must supply two arguments of the form: -a \"expression\", where a is for arithmetic, and expression is an expression of these two forms.");
    } else {
        if &args[0] == "-a" {
            // arithmetic
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
            if let Some(tokens) = parser.parse(&args[1]) {
                if tokens.len() == 0 {
                    println!("Error: Empty expression.");
                } else {
                    if let Some(tree) = parser.build_token_tree(&tokens) {
                        if let Some(value) = tree.evaluate() {
                            println!("Value: {}", value);
                        } else {
                            println!("Error: Unable to evaluate parse tree.");
                        }
                    } else {
                        println!("Error: Unable to build parse tree.");
                    }
                }
            } else {
                println!("Error: Unable to parse expression.");
            }
        } else {
            println!("Unsupported expression type.");
        }
    }
}