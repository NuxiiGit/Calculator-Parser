mod parser;

/// A structure used to define the valid operators to use within the equation.
#[allow(dead_code)]
pub struct Operator<'a, T> {
    symbol : &'a str,
    order : usize,
    f : fn(T, T) -> T
}
impl<'a, T> Operator<'a, T> {
    /// Constructs a new `Operator` instance.
    pub fn new(symbol : &'a str, order : usize, f : fn(T, T) -> T) -> Operator<'a, T> {
        Operator {
            symbol : symbol,
            order : order,
            f : f
        }
    }
}

/// Evaluates an equation and returns a `Result<T, &str>` where `T` is the output type and `&str` is an error string.
#[allow(dead_code)]
pub fn evaluate<'a, T>(expression : &str, operators : &[Operator<'a, T>]) -> Result<T, &'a str> {
    if !well_braced(expression) {
        return Err("Unable to parse: expression is not well-braced!");
    }
    let equation : String = expression
        .replace("[", "(")
        .replace("]", ")");
    let mut delimiters : Vec<&str> = Vec::new();
    // compile a list of operator symbols
    delimiters.push("(");
    delimiters.push(")");
    for operator in operators {
        let symbol : &str = operator.symbol;
        if symbol.contains("(")
        || symbol.contains(")")
        || symbol.contains("[")
        || symbol.contains("]") {
            return Err("Code Error: Illegal use of brace within operator definition!");
        }
        delimiters.push(symbol);
    }
    let tokens : Vec<String> = parser::collect_tokens(&equation, &delimiters);
    // convert infix to postfix
    
    Err("Not implemented.")
}

/// Returns whether the equation is well-braced.
/// Legal braces include `( )`, `[ ]`, `{ }`.
#[allow(dead_code)]
fn well_braced(expression : &str) -> bool {
    #[derive(PartialEq)]
    enum BraceType {
        Round,
        Square,
        Curly
    };
    let mut stack : Vec<BraceType> = Vec::new();
    for ch in expression.chars() {
        let brace : BraceType;
        let open : bool;
        match ch {
            '(' => { brace = BraceType::Round;  open = true;  },
            ')' => { brace = BraceType::Round;  open = false; },
            '[' => { brace = BraceType::Square; open = true;  },
            ']' => { brace = BraceType::Square; open = false; },
            '{' => { brace = BraceType::Curly;  open = true;  },
            '}' => { brace = BraceType::Curly;  open = false; },
             _  => { continue; }
        }
        if open {
            stack.push(brace);
        } else {
            match stack.pop() {
                Some(ancestor) => {
                    if ancestor == brace {
                        continue;
                    }
                },
                None => {}
            }
            return false;
        }
    }
    stack.len() == 0
}