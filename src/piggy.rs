pub fn to_pig_latin(text: String) -> String {
    let mut translated_words = vec![];
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    for word in text.split_whitespace() {
        let mut first_char: char = 'a';
        for c in  word.chars() {
            first_char = c;
            break;
        }
        if vowels.contains(&first_char) {
            let comb = format!("{}hey", word);
            translated_words.push(comb);
        }
        else {
            let comb = format!("{}{}ay", &word[1..], first_char.to_lowercase());
            translated_words.push(comb);
        }
    }

    // Hatway imetay ishay ithay?
    let output = translated_words.join(" ");
    output
}