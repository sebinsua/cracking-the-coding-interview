use std::collections::HashSet;

fn is_unique(string: &str) -> bool {
    let mut chars: HashSet<char> = HashSet::new();
    for c in string.chars() {
        if chars.contains(&c) {
            return false
        } else {
            chars.insert(c);
        }
    }
    true
}

fn is_unique2(string: &str) -> bool {
    if string.len() > 128 {
        return false
    }

    let mut chars: [bool; 128] = [false; 128];
    for c in string.chars() {
        let idx = c as usize;
        if chars[idx] {
            return false;
        }
        chars[idx] = true;
    }
    true
}

#[cfg(test)]
mod tests {

    mod is_unique {
        use is_unique;

        #[test]
        fn has_no_unique_characters() {
            assert_eq!(true, is_unique("abcdef"));
            assert_eq!(true, is_unique("ghijkl"));
            assert_eq!(true, is_unique("mnopqr"));
            assert_eq!(true, is_unique("stuvwx"));
            assert_eq!(true, is_unique("yz1234"));
        }

        #[test]
        fn has_unique_characters() {
            assert_eq!(false, is_unique("aabcdef"));
            assert_eq!(false, is_unique("abcdebf"));
            assert_eq!(false, is_unique("abcdefc"));
            assert_eq!(false, is_unique("eabcdef"));
            assert_eq!(false, is_unique("afbcdef"));
        }
    }

    mod is_unique2 {
        use is_unique2;

        #[test]
        fn has_no_unique_characters() {
            assert_eq!(true, is_unique2("abcdef"));
            assert_eq!(true, is_unique2("ghijkl"));
            assert_eq!(true, is_unique2("mnopqr"));
            assert_eq!(true, is_unique2("stuvwx"));
            assert_eq!(true, is_unique2("yz1234"));
        }

        #[test]
        fn has_unique_characters() {
            assert_eq!(false, is_unique2("aabcdef"));
            assert_eq!(false, is_unique2("abcdebf"));
            assert_eq!(false, is_unique2("abcdefc"));
            assert_eq!(false, is_unique2("eabcdef"));
            assert_eq!(false, is_unique2("afbcdef"));
        }
    }

}
