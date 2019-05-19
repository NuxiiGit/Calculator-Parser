mod parser;
mod input;

use parser::*;

fn main() {
    let mut parser : Parser<f64> = Parser::new();
    // add operators of parser
    parser.add_op(Operator::new("(_)", 3, |args| args[0]));
    parser.add_op(Operator::new("|_|", 2, |args| {
        if args[0] >= 0.0 {
            args[0]
        } else {
            -args[0]
        }
    }));
    parser.add_op(Operator::new("_*_", 1, |args| args[0] * args[1]));
    parser.add_op(Operator::new("_/_", 1, |args| args[0] / args[1]));
    parser.add_op(Operator::new("_+_", 0, |args| args[0] + args[1]));
    parser.add_op(Operator::new("_-_", 0, |args| args[0] - args[1]));
    // tests
    print!("Your symbols are:");
    for symbol in parser.symbols() {
        print!(" {},", symbol);
    }
    println!();
    println!("Your operators are:");
    for operator in parser.operators() {
        println!("pattern={}, precedence={}, (post={}, bracket={}, pre={})", operator.pattern(), operator.precedence(), operator.is_post(), operator.is_bracket(), operator.is_pre());
    }
    // parse
    let expression : String = input::read_buffer();
    match parser.parse(&expression) {
        Ok(tokens) => {
            if tokens.len() == 0 {
                println!("Empty expression.");
            } else {
                for token in tokens {
                    match token {
                        Token::Symbol(symbol) => {
                            println!("Symbol: {}", symbol);
                        },
                        Token::Value(value) => {
                            println!("Value: {}", value);
                        }
                    }
                }
            }
        },
        Err(msg) => {
            println!("Error: {}", msg);
        }
    }
}