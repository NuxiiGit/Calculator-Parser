mod parser;
mod input;

fn main() {
    let expression : String = input::read_buffer();
    /*let mut operators : Vec<parser::Operator<f64>> = Vec::new();
    // define operators
    operators.push(parser::Operator::new("*", 1, |a, b| a * b));
    operators.push(parser::Operator::new("/", 1, |a, b| a / b));
    operators.push(parser::Operator::new("+", 0, |a, b| a + b));
    operators.push(parser::Operator::new("-", 0, |a, b| a - b));
    */
}