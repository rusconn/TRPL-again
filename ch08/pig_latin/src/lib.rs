pub fn pig_latin(word: &str) -> Option<String> {
    let mut chars = word.chars();
    let head = chars.next()?;

    let altered = if "aeiou".contains(head) {
        format!("{}-hay", String::from(word))
    } else {
        format!("{}-{}ay", chars.as_str(), head)
    };

    Some(altered)
}

#[test]
fn test_pig_latin() {
    assert_eq!(pig_latin(""), None);
    assert_eq!(pig_latin("first"), Some(String::from("irst-fay")));
    assert_eq!(pig_latin("apple"), Some(String::from("apple-hay")));
}
