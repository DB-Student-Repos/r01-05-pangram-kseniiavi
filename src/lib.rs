/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let l_sentence = sentence.to_lowercase();
    let w_sentence = l_sentence.replace(" ", "");
    let mut alphabet = "abcdefghijklmnopqrstuvwxyz".to_string();
    for i in w_sentence.chars() {
        alphabet = alphabet.replace(i, ""); 
    }
    alphabet.is_empty()

    //unimplemented!("Is {sentence} a pangram?");
}
