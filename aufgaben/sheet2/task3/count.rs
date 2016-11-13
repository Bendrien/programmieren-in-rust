fn count(str: &str, char: char) -> usize {
    str.chars()
        .filter(|&c| c == char)
        .count()
}

#[test]
fn count_string() {
    assert_eq!(count("peter", 'e'), 2);
}
