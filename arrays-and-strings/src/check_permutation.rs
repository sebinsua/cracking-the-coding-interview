
/*
 * Given two strings, write a method to decide if one is a permutation of the other.
 */
fn check_permutation(string1: &str, string2: &str) -> bool {
    if string1.len() != string2.len() {
        return false
    }

    let mut chars: [i32; 128] = [0; 128];
    for c in string1.chars() {
        let idx = c as usize;
        chars[idx] = chars[idx] + 1;
    }

    for c in string2.chars() {
        let idx = c as usize;
        chars[idx] = chars[idx] - 1;
        if chars[idx] < 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {

    use check_permutation;

    #[test]
    fn can_detect_when_strings_are_permutations() {
        assert_eq!(true, check_permutation("abcdef", "abcdef"));
        assert_eq!(true, check_permutation("abcdef", "fedabc"));
        assert_eq!(true, check_permutation("abcdef", "abdcfe"));
        assert_eq!(true, check_permutation("abcdef", "feadbc"));
        assert_eq!(true, check_permutation("hellot", "thello"));
    }

    #[test]
    fn can_detect_when_strings_are_not_permutations() {
        assert_eq!(false, check_permutation("abcdef", ""));
        assert_eq!(false, check_permutation("abc", "def"));
        assert_eq!(false, check_permutation("abcdef", "abzdef"));
        assert_eq!(false, check_permutation("abcdef", "abcdeg"));
        assert_eq!(false, check_permutation("abcdef", "abcddd"));
    }

}
