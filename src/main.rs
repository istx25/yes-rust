use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        let printable_str = get_printable_str(None);
        yes(printable_str);
    }
    
    let arg = &args[0];
    let arg = arg.to_string();
    let printable_str = get_printable_str(Some(arg));
    yes(printable_str);
}
    
fn yes(input: String) {
    loop {
        println!("{}", input);
    }
}

fn get_printable_str(input: Option<String>) -> String {
    match input {
        Some(custom_str) => custom_str,
        None => String::from("y"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_none_get_printable_str() {
        let input: Option<String> = None;
        let output = get_printable_str(input);

        assert_eq!(output, "y");
    }

    #[test]
    fn test_some_get_printable_str() {
        let s = String::from("s");
        let input: Option<String> = Some(s);
        let output = get_printable_str(input);

        assert_eq!(output, "s");
    }
}
