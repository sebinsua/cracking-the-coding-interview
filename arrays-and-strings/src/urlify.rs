use std::vec::Vec;

fn urlify(string: &str) -> String {
    let whitespace_count = string.chars().fold(0, |acc, c| if c == ' ' { acc + 1 } else { acc });
    let new_string_length = string.len() + (whitespace_count * 3);

    // This could be done simpler.
    // Originally, I tried to use a fixed array, but that couldn't receive a dynamic length...
    let mut chars: Vec<char> = Vec::with_capacity(new_string_length);
    for c in string.trim().chars().rev() {
        if c == ' ' {
            chars.insert(0, '0');
            chars.insert(0, '2');
            chars.insert(0, '%');
        } else {
            chars.insert(0, c);
        }
    }

    chars.into_iter().collect()
}

#[cfg(test)]
mod tests {

    use urlify;

    #[test]
    fn can_replace_whitespace() {
        assert_eq!("hey%20there%20you", urlify("hey there you"));
        assert_eq!("expect%20%20this%20%20to%20%20work", urlify("expect  this  to  work  "));
    }

}
