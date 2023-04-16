/// lib.rs
/// Bradley Thompson
/// Home for all common helpers and/or types that I like to re-use accross play projects.
use text_colorizer::{Color, Colorize};
use std::str::FromStr;

/// Helper that will build a horizontal line out of hyphens for use in CLI programs.
/// When a color is provided, it will print out the line to cleanup common usage.
/// Mostly just returns to simplify testing.
///
/// # Arguments
///
/// * `length` - How many hyphens long the line should be.
/// * `color` - Optional color choice for the line; line only prints if a color is provided.
pub fn horizontal_sep(length: u8, color: Option<Color>) -> String {
    let mut container = String::with_capacity(length as usize);
    for _ in 0..length {
        container.push('-')
    }
    if let Some(c) = color {
        let colored = container.color(c);
        println!("{colored}")
    }
    container // Ownership moved to whatever captures this.
}

/// Helper to parse a string slice as a u64, or panic!
pub fn parse_num<T: FromStr>(s: &str) -> T {
    // TODO: Should prob just move this to my helper lib
    match s.parse() {
        Ok(num) => num,
        Err(_) => {
            let parse_error = format!("Invalid input detected: {}", s).bright_red();
            eprintln!("{parse_error}");
            panic!("Invalid input");
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horizontal_sep() {
        let result = horizontal_sep(10, None);
        assert_eq!(result.len(), 10);
    }

    #[test]
    fn test_horizontal_sep_color_does_not_alter_return() {
        let result = horizontal_sep(10, Some(Color::Blue));
        assert_eq!(result.len(), 10);
    }

    #[test]
    fn test_parse_num() {
        assert_eq!(parse_num::<u64>("12"), 12);
        assert_eq!(parse_num::<i8>("-2"), -2);
    }

    #[test]
    #[should_panic(expected = "Invalid input")]
    fn test_parse_num_handles_invalid_input() {
        // Note: see https://doc.rust-lang.org/book/ch11-01-writing-tests.html#checking-for-panics-with-should_panic
        parse_num::<u64>("hello world");
    }
}
