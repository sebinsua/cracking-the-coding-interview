fn string_compression(string: &str) -> String {
    let mut compressed_string = String::new();

    let mut chr: char = string.chars().nth(0).unwrap_or('\0');
    let mut count: u32 = 0;
    for (idx, c) in string.chars().enumerate() {
        if c != chr {
            compressed_string.push_str(&format!("{}{}", chr, count));
            chr = c;
            count = 1;
        } else {
            count = count + 1;
        }

        if (idx + 1) >= string.len() {
            compressed_string.push_str(&format!("{}{}", chr, count));
        }
    }

    if compressed_string.len() < string.len() {
        return compressed_string;
    }

    return string.to_string();
}

#[cfg(test)]
mod tests {

    use string_compression;

    #[test]
    fn is_compressed() {
        assert_eq!("a1b2c5", string_compression("abbccccc"));
        assert_eq!("a2b2c3", string_compression("aabbccc"));
        assert_eq!("a3b2c1d3", string_compression("aaabbcddd"));
        assert_eq!("z1e1b1r5a3", string_compression("zebrrrrraaa"));
        assert_eq!("f3a3i3t3h3l3e3s3", string_compression("fffaaaiiittthhhllleeesss"));
        assert_eq!("h1u2m1a8n1", string_compression("huumaaaaaaaan"));
    }

    #[test]
    fn is_not_compressed() {
        assert_eq!("abc", string_compression("abc"));
        assert_eq!("hellothere", string_compression("hellothere"));
        assert_eq!("isee", string_compression("isee"));
    }

}
