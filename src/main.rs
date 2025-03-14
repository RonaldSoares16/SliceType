fn main() {
    let test_phrase = String::from("Test to transform string to bite and get the first word");
    let result = get_first_word(&test_phrase);
    println!("{}", result);
}

fn get_first_word(phrase: &String) -> usize {
    let bytes = phrase.as_bytes(); //saves the string reference as an array of bytes
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return i;
        }
    }

    phrase.len()
}
