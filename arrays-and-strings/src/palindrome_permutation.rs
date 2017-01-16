fn palindrome_permutation(string: &str) -> bool {
    if string.len() <= 1 {
        return true;
    }

    let mut chars: [u32; 128] = [0; 128];
    for c in string.chars() {
        let idx = c as usize;
        chars[idx] = chars[idx] + 1;
    }

    let mut odds = 0;
    for n in chars.iter() {
        if *n > 0 && n % 2 != 0 {
            odds = odds + 1;
        }
    }

    if odds > 1 {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {

    use palindrome_permutation;

    #[test]
    fn can_form_palindrome() {
        assert_eq!(true, palindrome_permutation("aaaaa"));
        assert_eq!(true, palindrome_permutation("abcabc"));
        assert_eq!(true, palindrome_permutation("abcdedcba"));
        assert_eq!(true, palindrome_permutation("123123456654"));
        assert_eq!(true, palindrome_permutation("palindrome1palindrome"));
    }

    #[test]
    fn can_not_form_palindrome() {
        assert_eq!(false, palindrome_permutation("abcde"));
        assert_eq!(false, palindrome_permutation("ababbz"));
        assert_eq!(false, palindrome_permutation("hey there you"));
        assert_eq!(false, palindrome_permutation("broken"));
        assert_eq!(false, palindrome_permutation("this will fail"));
    }

}
