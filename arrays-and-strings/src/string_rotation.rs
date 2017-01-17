/*
 * Assume you have a method isSubstring which checks if one word is a substring of another. Given
 * two strings, s1 and s2, write code to check if s2 is a rotation of s1 using only one call to
 * isSubstring (e.g. "waterbottle" is a rotation of "erbottlewat").
 */
fn string_rotation(string1: &str, string2: &str) -> bool {
    if string1.len() != string2.len() {
        return false
    }

    let first_character = string2.chars().nth(0).unwrap_or('\0');
    let rotation_idx = match string1.find(|chr| chr == first_character) {
        Some(idx) => idx,
        None => return false
    };

    let rotated_string = format!("{}{}", &string1[rotation_idx..], &string1[..rotation_idx]);

    if string2.contains(&rotated_string) {
        return true
    }

    false
}

#[cfg(test)]
mod tests {

    use string_rotation;

    #[test]
    fn is_rotation() {
        assert_eq!(true, string_rotation("cab", "abc"));
        assert_eq!(true, string_rotation("rehello the", "hello there"));
        assert_eq!(true, string_rotation("believe thiscannot ", "cannot believe this"));
        assert_eq!(true, string_rotation("not believe thiscan", "cannot believe this"));
        assert_eq!(true, string_rotation("youhey i just met ", "hey i just met you"));
        assert_eq!(true, string_rotation("y i just met youhe", "hey i just met you"));
    }

    #[test]
    fn is_not_rotation() {
        assert_eq!(false, string_rotation("abc", "cba"));
        assert_eq!(false, string_rotation("rehello t he", "hel lo there"));
        assert_eq!(false, string_rotation(" believe this cannot", "cannot believe this"));
        assert_eq!(false, string_rotation("this cannot believe ", "cannot believe this"));
        assert_eq!(false, string_rotation("you met just i hey ", "hey i just met you"));
    }

}
