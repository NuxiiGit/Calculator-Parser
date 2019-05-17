static BRACES : &[(&str, &str)] = &[("(",")"),("[","]")];

/// A structure used to define the operators of an equation.
#[allow(dead_code)]
pub struct Operator<T> {
    symbol : String,
    priority : usize,
    f : fn(T, T) -> T
}

impl<T> Operator<T> {
    /// Constructs a new `Operator` instance.
    #[allow(dead_code)]
    pub fn new(symbol : &str, priority : usize, f : fn(T, T) -> T) -> Operator<T> {
        if symbol == "" {
            panic!("Operator symbol must be non-empty!");
        } else {
            for (left, right) in BRACES {
                if symbol.contains(left)
                || symbol.contains(right) {
                    panic!("Illegal use of brace within operator definition!");
                }
            }
        }
        Operator {
            symbol : String::from(symbol),
            priority : priority,
            f : f
        }
    }
}

/// A private enum used to store tokens together.
enum Token<'a, T> {
    OpenBracket(),
    CloseBracket(),
    Type(T),
    Operator(&'a Operator<T>)
}

/// Parses and verifies input, then returns a `Result<T, &str>`.
#[allow(dead_code)]
pub fn parse<'a, T>(expression : &str, operators : &[Operator<T>]) -> Result<T, &'a str> where
        T : std::str::FromStr,
        T : std::fmt::Display {
    let tokens : Vec<String> = collect_tokens(
        &expression
            .replace("\n","")
            .replace("\r","")
            .replace(" ",""),
        operators
    );
    let mut infix : Vec<Token<T>> = Vec::new();
    let mut postfix : Vec<Token<T>> = Vec::new();
    // check the expression is well-braced
    if !(| | {
        let mut stack : Vec<&(&str, &str)> = Vec::new();
        for token in &tokens {
            for brace in BRACES {
                if token == brace.0
                || token == brace.1 {
                    if token == brace.0 {
                        stack.push(brace);
                    } else {
                        match stack.pop() {
                            Some(ancestor) => {
                                if ancestor != brace {
                                    return false;
                                }
                            },
                            None => {
                                return false;
                            }
                        }
                    }
                    break;
                }
            }
        }
        stack.len() == 0
    })() {
        return Err("Expression is not well-braced");
    }
    // parse tokens
    'outer : for token in &tokens {
        for brace in BRACES {
            if token == brace.0
            || token == brace.1 {
                if token == brace.0 {
                    infix.push(Token::OpenBracket());
                } else {
                    infix.push(Token::CloseBracket());
                }
                continue 'outer;
            }
        }
        for operator in operators {
            if token == &operator.symbol {
                infix.push(Token::Operator(operator));
                continue 'outer;
            }
        }
        match token.parse::<T>() {
            Ok(value) => {
                infix.push(Token::Type(value))
            },
            Err(_) => {
                println!("{}{}", token,"a");
                return Err("Unable to parse token.");
            }
        }
    }
    // convert infix to postfix
    let mut operator_stack : Vec<Token<T>> = Vec::new();
    for token in infix {
        match token {
            Token::OpenBracket() => {
                operator_stack.push(token)
            },
            Token::CloseBracket() => {
                loop {
                    match operator_stack.pop() {
                        Some(Token::Operator(operator)) => {
                            postfix.push(Token::Operator(operator));
                        },
                        _ => {
                            break;
                        }
                    }
                }
            },
            Token::Operator(op1) => {
                match operator_stack.last() {
                    Some(Token::Operator(op2)) => {
                        if op2.priority > op1.priority {
                            loop {
                                match operator_stack.pop() {
                                    Some(Token::Operator(operator)) => {
                                        postfix.push(Token::Operator(operator));
                                    },
                                    _ => {
                                        break;
                                    }
                                }
                            }
                        }
                    },
                    _ => {}
                }
                operator_stack.push(token);
            },
            _ => {
                postfix.push(token);
            }
        }
    }
    while !operator_stack.is_empty() {
        match operator_stack.pop() {
            Some(Token::Operator(operator)) => {
                postfix.push(Token::Operator(operator));
            },
            _ => {}
        }
    }
    for token in &postfix {
        match token {
            Token::Operator(operator) => {
                println!("Operator: {}", operator.symbol);
            },
            Token::Type(value) => {
                println!("Type:     {}", value);
            }
            _ => {}
        }
    }
    Err("Not implemented!")
}

/// Returns a `Vec<String>`
/// Types of braces are found within `BRACES`.
#[allow(dead_code)]
pub fn collect_tokens<T>(expression : &str, operators : &[Operator<T>]) -> Vec<String> {
    let mut margins : Vec<(usize, &str)> = Vec::new();
    let mut symbols : Vec<&str> = Vec::new();
    for brace in BRACES {
        symbols.push(brace.0);
        symbols.push(brace.1);
    }
    for operator in operators {
        symbols.push(&operator.symbol);
    }
    for symbol in symbols {
        'outer : for (i, _) in expression.match_indices(symbol) { 
            for (j, delimiter) in &margins {
                match j { &v => {
                    if i >= v
                    && i <  v + delimiter.len() {
                        continue 'outer;
                    }
                }}
            }
            margins.push((i, symbol));
            break;
        }
    }
    margins.sort_by(|a, b| {
        use std::cmp::Ordering;
        if a.0 < b.0 {
            Ordering::Less
        } else if a.0 > b.0 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });
    let mut tokens : Vec<String> = Vec::new();
    let mut base : usize = 0;
    for (i, delimiter) in margins {
        let substring : String = expression.chars()
            .skip(base)
            .take(i-base)
            .collect();
        if &substring != "" {
            tokens.push(substring);
        }
        base = i + delimiter.len();
        tokens.push(String::from(delimiter));
    }
    // push final element
    let last : String = expression.chars()
        .skip(base)
        .take(expression.len() - base)
        .collect();
    if &last != "" {
        tokens.push(last);
    }
    tokens
}