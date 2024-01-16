fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut first_index: Option<usize> = None;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            match first_index {
                Some(j) => return &s[j..i],
                None => first_index = Some(i+1),
            }
        }
    }

    &s[..]
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let phrase = String::from("This is a test phrase");
    let long_word = String::from("Hippopotomonstrosesquippedaliophobia");

    let first = first_word(&phrase);
    let second = second_word(&phrase);
    let full_word = first_word(&long_word);
    let full_word_2 = second_word(&long_word);

    assert_eq!(&phrase[0..4], first);
    assert_eq!(&phrase[5..7], second);
    assert_eq!(long_word, full_word);
    assert_eq!(long_word, full_word_2);
}
