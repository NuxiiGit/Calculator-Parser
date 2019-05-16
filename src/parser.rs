static BRACES : &[(&str, &str)] = &[("(",")"),("[","]")];

/// A structure used to define the operators of an equation.
#[allow(dead_code)]
pub struct Operator<'a, T> {
    symbol : &'a str,
    priority : usize,
    f : fn(T, T) -> T
}

impl<'a, T> Operator<'a, T> {
    /// Constructs a new `Operator` instance.
    #[allow(dead_code)]
    pub fn new(symbol : &'a str, priority : usize, f : fn(T, T) -> T) -> Operator<'a, T> {
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
            symbol : symbol,
            priority : priority,
            f : f
        }
    }
}

/// Returns a `Vec<String>`
/// Types of braces are found within `BRACES`.
#[allow(dead_code)]
pub fn collect_tokens<'a, T>(expression : &str, operators : &[Operator<'a, T>]) -> Vec<String> {
    let mut margins : Vec<(usize, &str)> = Vec::new();
    let mut symbols : Vec<&str> = Vec::new();
    for brace in BRACES {
        symbols.push(brace.0);
        symbols.push(brace.1);
    }
    for operator in operators {
        symbols.push(operator.symbol);
    }
    for symbol in symbols {
        for (i, _) in expression.match_indices(symbol) {
            'avoid_duplicates : loop {
                for (j, delimiter) in &margins {
                    match j { &v => {
                        if i >= v
                        && i <  v + delimiter.len() {
                            break 'avoid_duplicates;
                        }
                    }}
                }
                margins.push((i, symbol));
                break;
            }
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

/// Returns whether an expression is well-braced.
/// Types of braces are found within `BRACES`.
#[allow(dead_code)]
pub fn well_braced(tokens : &[String]) -> bool {
    let mut stack : Vec<&(&str, &str)> = Vec::new();
    for token in tokens {
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
}

// Evaluates a collection of tokens and returns a result `T`.
//pub fn evaluate<'a, T>(tokens : &[String], operators : &[Operator<'a, T>]) -> T {
//  let accumulator : T;
//}