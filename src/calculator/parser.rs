/// Returns a `Vec<String>` of tokens, separated by an array of delimiters.
#[allow(dead_code)]
pub fn collect_tokens(expression : &str, delimiters : &[&str]) -> Vec<String> {
    let mut margins : Vec<(usize, String)> = Vec::new();
    for &delimiter in delimiters {
        if delimiter == "" {
            continue;
        }
        for (i, _) in expression.match_indices(&delimiter) {
            let mut duplicate : bool = false;
            for (j, d) in &margins {
                if i >= j.clone() && i < j.clone() + d.len() {
                    duplicate = true;
                    break;
                }
            }
            if !duplicate {
                margins.push((i, String::from(delimiter)));
            }
        }
    }
    margins.sort_by(|a, b| {
        use std::cmp::Ordering;
        if a.0 < b.0 {
            Ordering::Less
        } else if a.0 > b.0 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });
    let mut tokens : Vec<String> = Vec::new();
    let mut base : usize = 0;
    for (i, delimiter) in margins {
        let substring : String = expression
            .chars()
            .skip(base)
            .take(i-base)
            .collect();
        if &substring != "" {
            tokens.push(substring);
        }
        base = i + delimiter.len();
        tokens.push(delimiter);
    }
    // add final element
    let last : String = expression
        .chars()
        .skip(base)
        .take(expression.len()-base)
        .collect();
    if &last != "" {
        tokens.push(last);
    }
    tokens
}
