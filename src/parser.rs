/// A structure used to parse input.
#[allow(dead_code)]
pub struct Parser<T> {
    operators : Vec<Operator<T>>
}
impl<T> Parser<T> {
    /// Constructs a new `Parser` instance.
    #[allow(dead_code)]
    pub fn new() -> Parser<T> {
        Parser {
            operators : Vec::new()
        }
    }

    /// Consumes an operator and adds it to the parser's operator vector.
    /// The vector is sorted in order of preceedence.
    /// Panics when the input operator contains a duplicate symbol.
    #[allow(dead_code)]
    pub fn add_op<'a>(&mut self, operator : Operator<T>) {
        for symbol in operator.symbols() {
            if self.symbols().contains(&symbol) {
                panic!("Cannot have ambiguous operator patterns.");
            }
        }
        self.operators.push(operator);
        self.operators.sort_by(|a, b| {
            use std::cmp::Ordering;
            let a : usize = a.precedence();
            let b : usize = b.precedence();
            if a < b {
                Ordering::Greater
            } else if a > b {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });
    }

    /// Takes an operator id and attempts to remove it from the parser.
    /// Returns `panics` if the operator does not exist, so you should always use `operators<T>()` to locate the operator you wish to remove before calling this.
    #[allow(dead_code)]
    pub fn remove_op<'a>(&mut self, index : usize) {
        self.operators.remove(index);
    }

    /// Splits a `&str` expression into a `Vec<Token>` of tokens.
    #[allow(dead_code)]
    pub fn parse<'a>(&self, expression : &str) -> Result<Vec<Token<T>>, &'a str> where
            T : std::str::FromStr {
        let expression : &str = &expression
                .replace("\n", "")
                .replace("\r", "")
                .replace("\t", "")
                .replace(" ", "");
        let mut tokens : Vec<Token<T>> = Vec::new();
        let mut regions : Vec<(usize, String)> = Vec::new();
        // compile symbol regions
        for symbol in self.symbols() {
            'outer:
            for (i, _) in expression.match_indices(&symbol) {
                for (j, other) in &regions {
                    let j : usize = j.to_owned();
                    if i >= j
                    && i < j + other.len() {
                        continue 'outer; // ignore overlap
                    }
                }
                regions.push((i, symbol.clone()));
            }
        }
        regions.sort_by(|(a, _), (b, _)| {
            use std::cmp::Ordering;
            if a < b {
                Ordering::Less
            } else if a > b {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
        // parse values and add them to the token vector
        let mut base : usize = 0;
        let last : usize = expression.len();
        for (i, symbol) in regions {
            if i > base {
                match self.parse_value(expression, base, i) {
                    Ok(value) => tokens.push(Token::Value(value)),
                    Err(msg) => return Err(msg)
                }
            }
            base = i + symbol.len();
            tokens.push(Token::Symbol(symbol));
        }
        if last > base {
            match self.parse_value(expression, base, last) {
                Ok(value) => tokens.push(Token::Value(value)),
                Err(msg) => return Err(msg)
            }
        }
        Ok(tokens)
    }

    /// Returns a `Vec<String>` of all possible operator symbols, organised from shortest to longest with no duplicates.
    #[allow(dead_code)]
    pub fn symbols(&self) -> Vec<String> {
        let mut symbols : Vec<String> = Vec::new();
        for operator in &self.operators {
            for symbol in operator.symbols() {
                if !symbols.contains(&symbol) {
                    symbols.push(symbol.to_owned());
                }
            }
        }
        symbols.sort_by(|a, b| {
            // sort in descending order
            use std::cmp::Ordering;
            if b.len() < a.len() {
                Ordering::Less
            } else if b.len() > a.len() {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
        symbols
    }

    /// Returns a `Vec<&Operator<T>>` of references to the operators within the parser
    #[allow(dead_code)]
    pub fn operators(&self) -> Vec<&Operator<T>> {
        let mut operators : Vec<&Operator<T>> = Vec::new();
        for operator in &self.operators {
            operators.push(operator);
        }
        operators
    }

    /// Parses a single value of an expression between a `start` and `end` index.
    /// Returns a `Result<Option<T>, &str>`. `Ok(None)` is returned when the substring results in an empty string.
    #[allow(dead_code)]
    pub fn parse_value<'a>(&self, expression : &str, start : usize, end : usize) -> Result<T, &'a str> where
            T : std::str::FromStr {
        if start >= end {
            return Err("Invalid expression substring.");
        }
        let substring : String = expression
                .chars()
                .skip(start)
                .take(end - start)
                .collect();
        match substring.parse::<T>() {
            Ok(value) => Ok(value),
            _ => Err("Unable to parse expression value.")
        }
    }
}

/// A recursive data type which is used to represent a parse tree.
#[allow(dead_code)]
#[derive(Clone)]
pub enum ParseTree<T> {
    Leaf(T),
    Node(Operator<T>, Vec<ParseTree<T>>)
}

/// A structure used to define generic operators.
#[allow(dead_code)]
#[derive(Clone)]
pub struct Operator<T> {
    postfix : bool,
    prefix : bool,
    symbols : Vec<String>,
    precedence : usize,
    operation : fn(&[T]) -> T
}

impl<T> Operator<T> {
    /// Constructs a new `Operator` instance.
    #[allow(dead_code)]
    pub fn new<'a>(pattern : &str, precedence : usize, operation : fn(&[T]) -> T) -> Operator<T> {
        let mut postfix : bool = false;
        let mut prefix : bool = false;
        let mut symbols : Vec<String> = Vec::new();
        for (i, item) in pattern
                .replace(" ", "")
                .split("_")
                .enumerate() {
            if item == "" {
                if i == 0 {
                    // this means that the pattern started with an underscore "_something"
                    postfix = true;
                } else {
                    // this means that the pattern ended with an underscore "something_"
                    prefix = true;
                }
            } else {
                symbols.push(item.to_owned());
            }
        }
        Operator {
            postfix : postfix,
            prefix : prefix,
            symbols : symbols,
            precedence : precedence,
            operation : operation
        }
    }

    /// Returns a vctor of available symbols for this operator.
    #[allow(dead_code)]
    pub fn symbols(&self) -> Vec<String> {
        self.symbols.clone()
    }

    /// Returns whether the operator has postfix behaviour
    #[allow(dead_code)]
    pub fn is_post(&self) -> bool {
        self.postfix
    }

    /// Returns whether the operator has prefix behaviour
    #[allow(dead_code)]
    pub fn is_pre(&self) -> bool {
        self.prefix
    }

    /// Returns whether the operator has bracket behaviour
    #[allow(dead_code)]
    pub fn is_bracket(&self) -> bool {
        // more than one symbol means that the operator has a pattern of (at least) the form "s1_s2" where "s1" and "s2" are symbols.
        // for example the modulus operator is a bracket operator because it has the pattern "|_|".
        self.symbols.len() > 1
    }

    /// Returns the operator's pattern as a string.
    #[allow(dead_code)]
    pub fn pattern(&self) -> String {
        let mut pattern : String;
        if self.postfix {
            pattern = String::from("_");
        } else {
            pattern = String::new();
        }
        for (i, symbol) in self.symbols
                .iter()
                .enumerate() {
            if i != 0 {
                pattern.push('_');
            }
            pattern.push_str(symbol);
        }
        if self.prefix {
            pattern + "_"
        } else {
            pattern
        }
    }

    /// Returns a clone of the operator's precedence to the caller.
    #[allow(dead_code)]
    pub fn precedence(&self) -> usize {
        self.precedence.clone()
    }

    /// Calls the operator's operation on an input array of arguments of type `T`.
    #[allow(dead_code)]
    pub fn operate(&self, args : &[T]) -> T {
        (self.operation)(args)
    }
}

/// An enum used to collect values and symbols into a single parent.
#[allow(dead_code)]
pub enum Token<T> {
    Value(T),
    Symbol(String)
}