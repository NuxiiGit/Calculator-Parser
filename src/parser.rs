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
    pub fn parse(&self, expression : &str) -> Option<Vec<Token<T>>> where
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
                    Some(value) => tokens.push(Token::Value(value)),
                    None => return None
                }
            }
            base = i + symbol.len();
            tokens.push(Token::Symbol(symbol));
        }
        if last > base {
            match self.parse_value(expression, base, last) {
                Some(value) => tokens.push(Token::Value(value)),
                None => return None
            }
        }
        Some(tokens)
    }

    /// Simplifys an expression from a vector of tokens to a single token tree.
    #[allow(dead_code)]
    pub fn build_token_tree(&self, expression : &[Token<T>]) -> Option<Token<T>> where
            T : Clone,
            T : std::fmt::Display {
        let mut expression : Vec<Token<T>> = expression.to_owned();
        for operators in self.operator_precedence() {
            let mut operator : Option<&Operator<T>> = None;
            while let Some(i) = expression.iter().position(|x| {
                        if let Token::Symbol(symbol) = x {
                            for candidate in &operators {
                                let symbols : Vec<String> = candidate.symbols();
                                if symbol == &symbols[symbols.len() -1] {
                                    operator = Some(candidate);
                                    return true;
                                }
                            }
                        }
                        false
                    }) {
                let operator : &Operator<T> = operator?;
                let symbols : Vec<String> = operator.symbols();
                // since i = end, I'm going to iterate backwards to find the start.
                // This might seem wasteful, but the computation is easier in the
                // long-term since I don't have to constantly reverse the vectors
                // when creeping forwards.
                let mut start : usize = i;
                loop {
                    if let Token::Symbol(symbol) = &expression[start] {
                        if symbol == &symbols[0] {
                            break;
                        }
                    }
                    if start == 0 {
                        return None; // no start
                    }
                    start -= 1;
                }
                let mut end : usize = start + 1;
                let mut arguments : Vec<Token<T>> = Vec::new();
                // push start element
                if operator.is_post() {
                    if start == 0 {
                        return None;
                    }
                    start -= 1;
                    arguments.push(expression[start].to_owned());
                }
                // push nested elements
                let mut symbol_id : usize = 1;
                let mut subexpression : Vec<Token<T>> = Vec::new();
                loop {
                    if symbol_id >= symbols.len() {
                        break;
                    }
                    if end < expression.len() {
                        match &expression[end] {
                            Token::Symbol(symbol) => {
                                if symbol == &symbols[symbol_id] {
                                    // build subexpression
                                    if let Some(tree) = self.build_token_tree(&subexpression) {
                                        arguments.push(tree);
                                        subexpression.clear();
                                        symbol_id += 1;
                                    } else {
                                        return None;
                                    }
                                } else {
                                    subexpression.push(Token::Symbol(symbol.to_owned()));
                                }
                            },
                            other => subexpression.push(other.to_owned())
                        }
                        end += 1;
                    } else {
                        // exceeds bounds
                        return None;
                    }
                }
                // push end element
                if operator.is_pre() {
                    if end == expression.len() {
                        return None;
                    }
                    arguments.push(expression[end].to_owned());
                    end += 1;
                }
                // debug
                print!("Operator: {} Operands: ", operator.pattern());
                for item in &arguments {
                    match item {
                        Token::Symbol(symbol) => print!("Symbol={},",symbol),
                        Token::Value(value) => print!("Value={},", value),
                        Token::Tree(op, _) => print!("Tree={},", op.pattern())
                    }
                }
                println!();
                // remove this segment and replace it with a new token tree
                while end > start {
                    end -= 1;
                    expression.remove(start);
                }
                expression.insert(start, Token::Tree(operator.to_owned(), arguments))
            }
        }
        if expression.len() == 1 {
            Some(expression[0].to_owned())
        } else {
            None
        }
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

    /// Returns a `Vec<&Operator<T>>` of references to the operators within the parser.
    #[allow(dead_code)]
    pub fn operators(&self) -> Vec<&Operator<T>> {
        let mut operators : Vec<&Operator<T>> = Vec::new();
        for operator in &self.operators {
            operators.push(operator);
        }
        operators
    }

    /// Returns a `Vec<Vec<&Operator<T>>>` of references to operators which have the same precedence.
    #[allow(dead_code)]
    pub fn operator_precedence(&self) -> Vec<Vec<&Operator<T>>> {
        let mut collection : Vec<Vec<&Operator<T>>> = Vec::new();
        let mut precedence : Option<usize> = None;
        let mut operators : Vec<&Operator<T>> = Vec::new();
        for operator in &self.operators {
            if let Some(current) = precedence {
                if operator.precedence() != current {
                    collection.push(operators);
                    operators = Vec::new();
                }
            }
            precedence = Some(operator.precedence());
            operators.push(operator);
        }
        if operators.len() != 0 {
            collection.push(operators);
        }
        collection
    }

    /// Parses a single value of an expression between a `start` and `end` index.
    /// Returns a `Result<Option<T>, &str>`. `Ok(None)` is returned when the substring results in an empty string.
    #[allow(dead_code)]
    pub fn parse_value(&self, expression : &str, start : usize, end : usize) -> Option<T> where
            T : std::str::FromStr {
        if start < end {
            let substring : String = expression
                    .chars()
                    .skip(start)
                    .take(end - start)
                    .collect();
            match substring.parse::<T>() {
                Ok(value) => return Some(value),
                _ => {}
            }
        }
        None
    }
}

/// An enum used to collect values and symbols into a single parent.
/// Also acts as a recursive data type used to express a parse tree, i.e. a `Token::Tree`.
#[allow(dead_code)]
#[derive(Clone)]
pub enum Token<T> {
    Value(T),
    Symbol(String),
    Tree(Operator<T>, Vec<Token<T>>)
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

    /// Calls the operator's operation on an input array of arguments of type `T`.
    #[allow(dead_code)]
    pub fn operate(&self, args : &[T]) -> T {
        (self.operation)(args)
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

    /// Returns a vector of available symbols for this operator.
    #[allow(dead_code)]
    pub fn symbols(&self) -> Vec<String> {
        self.symbols.clone()
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
}