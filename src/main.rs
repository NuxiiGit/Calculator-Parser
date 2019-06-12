mod parser;
mod input;

use parser::*;

fn main() {
    // initialise parser
    let parser = build_parser!(
        "true"  => |_| 1.0,
        "false" => |_| 0.0;
        "(_)"   => |args| args[0],
        "|_|"   => |args| if args[0] >= 0.0 {args[0]} else {-args[0]};
        "_!"    => |args| {
            fn fact(n : f64) -> f64 {
                let n : f64 = n.floor();
                match n {
                    _ if n < 1.0 => 1.0,
                    _ => n * fact(n - 1.0)
                }
            };
            fact(args[0])
        };
        "_^_"    => |args| {
            let a : f64 = args[0];
            a.powf(args[1])
        };
        "_*_"   => |args| args[0] * args[1],
        "_/_"   => |args| args[0] / args[1],
        "_%_"   => |args| args[0] % args[1];
        "_+_"   => |args| args[0] + args[1],
        "_-_"   => |args| args[0] - args[1];
        "_>_"   => |args| if args[0] > args[1] {1.0} else {0.0},
        "_<_"   => |args| if args[0] < args[1] {1.0} else {0.0},
        "_>=_"  => |args| if args[0] >= args[1] {1.0} else {0.0},
        "_<=_"  => |args| if args[0] <= args[1] {1.0} else {0.0},
        "_=_"   => |args| if args[0] <= args[1] {1.0} else {0.0};
        "_/\\_" => |args| if (args[0] * args[1]) > 0.0 {1.0} else {0.0};
        "_\\/_" => |args| if (args[0] + args[1]) > 0.0 {1.0} else {0.0};
        "_->_"  => |args| if (args[0] > 0.0) && (args[1] <= 0.0) {0.0} else {1.0};
        "_?_:_" => |args| if args[0] > 0.0 {args[1]} else {args[2]};
    );
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