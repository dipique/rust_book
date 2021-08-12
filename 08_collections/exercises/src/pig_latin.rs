// if a word begins with a consonant or consonants, move it (them) to they end and add "ay"
// if it begins with a vowel, just add "hay" to the end of the word
// in either case, a dash is added before the new suffix
// don't worry about 'y' for now

static VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

pub fn to_pig_latin(val: &str) -> String {
    let first_vowel = val.chars()
        .position(|ch| VOWELS.contains(&ch)) // gets the first index where condition met
        .unwrap(); // dies if val == "b"
    let suffix_start = if first_vowel == 0 { "h" } else { &val[..first_vowel] };
    format!("{}-{}ay", &val[first_vowel..], suffix_start)
}