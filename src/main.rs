use std::io::{self, Read};

fn unindent_text(text: &str) -> String {
    let lines = text.lines().collect::<Vec<_>>();
    let min_indent = lines
        .iter()
        .filter(|line| line.len() > 0)
        .map(|line| line.len() - line.trim_start().len())
        .min()
        .unwrap_or(0);
    lines
        .iter()
        .map(|line| {
            if line.len() >= min_indent {
                &line[min_indent..]
            } else {
                line
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() {
    let mut text = String::new();
    io::stdin().read_to_string(&mut text).unwrap();
    println!("{}", unindent_text(&text));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unindent_text() {
        let input = "  This is indented.\n    This is more indented.\n\n  This is indented again.";
        let expected_output = "This is indented.\n  This is more indented.\n\nThis is indented again.";
        assert_eq!(unindent_text(input), expected_output);

        let input = "  This is indented.\n  This is also indented.\n\n\n  This is indented again.";
        let expected_output = "This is indented.\nThis is also indented.\n\n\nThis is indented again.";
        assert_eq!(unindent_text(input), expected_output);

        let input = "This is not indented.";
        let expected_output = "This is not indented.";
        assert_eq!(unindent_text(input), expected_output);
    }
}
