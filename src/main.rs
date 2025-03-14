use std::io::Bytes;

fn main() {
    let test_phrase = String::from("Rain formerly known as purple"); //phrase to be used
    let size = get_first_word_as_bytes(&test_phrase); // transforms the phrase into bytes and gets the first word
    let result = bytes_to_string(&test_phrase, size); // transforms the bytes back into a string
    let result2 = get_first_word(&test_phrase);
    println!("first method: {}", result);
    println!("second method: {}", result2);
}

fn get_first_word_as_bytes(phrase: &String) -> usize {
    let bytes = phrase.as_bytes(); //saves the string reference as an array of bytes
    for (i, &item) in bytes.iter().enumerate() { //gets the bytes till it reaches to a space
        if item == b' '{
            return i;
        }
    }
    phrase.len() //returns the phrase as the bytes of the first word
}

fn bytes_to_string(phrase: &String, bytes: usize) -> String {
    let mut vector_phrase = vec![];
    for i in 0..=bytes{
        vector_phrase.push(phrase.as_bytes()[i]); //adds the bytes to the vector
    }
    let x = String::from_utf8_lossy(&vector_phrase); //transforms byte vector into utf8 characters
    x.to_string()
}

fn get_first_word(phrase: &String) -> &str {
    let bytes = phrase.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &phrase[..i];
        }
    }
    &phrase[..]
}