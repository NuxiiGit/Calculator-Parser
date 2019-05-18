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
        for symbol in operator.pattern() {
            if self.symbols().contains(symbol) {
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
        for (i, symbol) in regions {
            match self.parse_value(expression, base, i) {
                Ok(Some(value)) => {
                    tokens.push(Token::Value(value));
                },
                Ok(None) => {},
                Err(msg) => {
                    return Err(msg);
                }
            }
            base = i + symbol.len();
            tokens.push(Token::Symbol(symbol));
        }
        match self.parse_value(expression, base, expression.len()) {
            Ok(Some(value)) => {
                tokens.push(Token::Value(value));
            },
            Ok(None) => {},
            Err(msg) => {
                return Err(msg);
            }
        }
        Ok(tokens)
    }

    /// Constructs and returns an `Result<ParseTree<T>, &str>` of the input `&[Token<T>]` array.
    #[allow(dead_code)]
    pub fn build_tree<'a>(&self, tokens : &[Token<T>]) -> Result<ParseTree<T>, &'a str> where
            T : std::clone::Clone {
        if tokens.len() == 0 {
            return Err("Empty token array.");
        }
        // re-format tokens into a vector of tokens containing either parse trees or symbols.
        let mut trees : Vec<Token<ParseTree<T>>> = Vec::new();
        for token in tokens {
            match token {
                Token::Value(value) => {
                    trees.push(Token::Value(ParseTree::Leaf(value.to_owned())));
                },
                Token::Symbol(symbol) => {
                    trees.push(Token::Symbol(symbol.to_owned()));
                }
            }
        }
        Err("Not implemented.")
    }

    /// Returns a `Vec<String>` of all possible operator symbols, organised from shortest to longest with no duplicates.
    #[allow(dead_code)]
    pub fn symbols(&self) -> Vec<String> {
        let mut symbols : Vec<String> = Vec::new();
        for operator in &self.operators {
            for symbol in operator.pattern() {
                if symbol != "_" // reserved argument symbol
                && !symbols.contains(symbol) {
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

    /// Parses a single value of an expression between a `start` and `end` index.
    /// Returns a `Result<Option<T>, &str>`. `Ok(None)` is returned when the substring results in an empty string.
    #[allow(dead_code)]
    pub fn parse_value<'a>(&self, expression : &str, start : usize, end : usize) -> Result<Option<T>, &'a str> where
            T : std::str::FromStr {
        let substring : String = expression
                .chars()
                .skip(start)
                .take(end - start)
                .collect();
        if start <= end
        && &substring != "" {
            match substring.parse::<T>() {
                Ok(value) => {
                    Ok(Some(value))
                },
                Err(_) => {
                    println!("{}, {}", start, end);
                    Err("Unable to parse expression value.")
                }
            }
        } else {
            Ok(None)
        }
    }
}

/// An enum used to collect values and operators into a single parent.
#[allow(dead_code)]
pub enum Token<T> {
    Value(T),
    Symbol(String)
}

/// A recursive data type which is used to represent a parse tree.
#[allow(dead_code)]
pub enum ParseTree<'a, T> {
    Leaf(T),
    Node(Operator<T>, &'a [ParseTree<'a, T>])
}

/// A structure used to define generic operators.
#[allow(dead_code)]
pub struct Operator<T> {
    pattern : Vec<String>,
    precedence : usize,
    operation : fn(&[T]) -> T
}

impl<T> Operator<T> {
    /// Constructs a new `Operator` instance.
    #[allow(dead_code)]
    pub fn new<'a>(pattern : &str, priority : usize, f : fn(&[T]) -> T) -> Operator<T> {
        Operator {
            pattern : {
                let mut split : Vec<String> = Vec::new();
                for item in pattern
                        .replace("_", " _ ")
                        .split(" ") {
                    if item != "" {
                        split.push(item.to_owned());
                    }
                }
                split
            },
            precedence : priority.to_owned(),
            operation : f
        }
    }

    /// Returns a reference to the operator's pattern.
    #[allow(dead_code)]
    pub fn pattern(&self) -> &[String] {
        &self.pattern
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