// Convert strings to pig latin. The first consonant of each word is
// moved to the end of the word and ay is added so first becomes irst-fay.
// Words that start with a vowel have hay added to the end instead
// (apple becomes apple-hay) Keep in mind the details about UTF-8 encoding
fn main() {
    let sentence = String::from("The quick fox jumps over the lazy brown dog");

    let words = sentence.split_whitespace();

    let vowels = vec!['a', 'e', 'i', 'o', 'u', 'y'];
    let mut new_sentence: String = String::new();

    for word in words {
        let mut chars = word.chars();
        let first_char = chars.next().unwrap();
        if vowels.contains(&first_char) {
            // vowels case: add "-hay"
            let new_word = String::from(word) + "-hay ";
            new_sentence = new_sentence + new_word.as_str();
        } else {
            // consonant case: irst-fay
            let mut new_word = String::from("");
            for c in chars {
                new_word.push(c);
            }
            let mut suffix = String::from("-");
            suffix.push(first_char);
            suffix = suffix + "ay ";
            new_word = new_word.to_string() + suffix.as_str();
            new_sentence = new_sentence + new_word.as_str();
       }
    }

    println!("{new_sentence}")
}
