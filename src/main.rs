mod calculator;
mod input;

fn main() {
    let expression : String = input::read_buffer();
    let mut operators : Vec<calculator::Operator<f64>> = Vec::new();
    // define operators
    operators.push(calculator::Operator::new("*", 1, |a, b| a * b));
    operators.push(calculator::Operator::new("/", 1, |a, b| a / b));
    operators.push(calculator::Operator::new("+", 0, |a, b| a + b));
    operators.push(calculator::Operator::new("-", 0, |a, b| a - b));
    // evaluate
    match calculator::evaluate(&expression.replace(" ",""), &operators) {
        Ok(value) => {
            println!("{} = {}", expression, value);
        },
        Err(msg) => {
            println!("{}", msg);
        }
    }
}