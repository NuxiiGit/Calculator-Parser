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
    #[allow(dead_code)]
    pub fn add<'a>(&mut self, operator : Operator<T>) {
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
    pub fn parse(&self, expression : &str) -> Vec<Token<T>> {
        let mut tokens : Vec<Token<T>> = Vec::new();

        tokens
    }

    /// Returns a `Vec<String>` of all possible operator symbols, organised from shortest to longest with no duplicates.
    #[allow(dead_code)]
    pub fn symbols(&self) -> Vec<String> {
        let mut symbols : Vec<String> = Vec::new();
        for operator in &self.operators {
            for symbol in operator.pattern() {
                if symbol != "_" { // reserved argument symbol
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
}

/// An enum used to collect values and operators into a single parent.
#[allow(dead_code)]
pub enum Token<T> {
    Value(T),
    Symbol(String)
}

/// A recursive data type which is used to represent a token tree.
#[allow(dead_code)]
pub enum TokenTree<'a, T> {
    Leaf(T),
    Node(Operator<T>, &'a [TokenTree<'a, T>])
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