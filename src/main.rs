mod input;
mod parser;

fn main() {
    let expression : String = input::read_buffer();
    let tokens : Vec<String> = parser::collect_tokens(&expression, &["->","+","-","*","/"]);
    for token in tokens {
        println!("{}", &token);
    }
}