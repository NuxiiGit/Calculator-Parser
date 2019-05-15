mod input;

fn main() {
    let expression : String = input::read_buffer();
    println!("{}", &expression);
}