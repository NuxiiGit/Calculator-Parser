/// A wrapper for standard input.
#[allow(dead_code)]
pub fn read_buffer() -> String {
    let mut buffer : String = String::new();
    match std::io::stdin().read_line(&mut buffer) {
        Ok(_) => {
            buffer
                .replace("\n","")
                .replace("\r","")
        },
        Err(_) => {
            String::new()
        }
    }
}

/// Return a `Vec<String>` of command line arguments.
/// Optionial arguments for defining range.
#[allow(dead_code)]
pub fn read_args(start : Option<usize>, end : Option<usize>) -> Vec<String> {
    let mut args : Vec<String> = Vec::new();
    let mut i : usize = 0;
    std::env::args().for_each(|x| {
        let mut valid : bool = true;
        if let Some(j) = start {
            if i < j {
                valid = false;
            }
        }
        if let Some(j) = end {
            if i >= j {
                valid = false;
            }
        }
        if valid {
            args.push(x);
        }
        i += 1;
    });
    args
}

/// Return a single `String` of concatenated command line arguments.
/// Optionial arguments for defining range.
#[allow(dead_code)]
pub fn read_args_single(start : Option<usize>, end : Option<usize>) -> String {
    let mut args : String = String::from("");
    for arg in read_args(start, end) {
        if args.len() > 0 {
            args.push(' ');
        }
        args.push_str(&arg);
    }
    args
}