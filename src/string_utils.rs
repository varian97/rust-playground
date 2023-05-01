pub const VOWELS: [char; 10] = ['a', 'i', 'u', 'e', 'o', 'A', 'I', 'U', 'E', 'O'];
// can also do:
// pub const VOWELS_SLICE: &[char] = &['a', 'i', 'u', 'e', 'o', 'A', 'I', 'U', 'E', 'O'];

pub fn convert_to_pig_latin(s: &mut String) {
    let first_letter = match s.chars().nth(0) {
        Some(ch) => ch,
        None => ' ',
    };

    if VOWELS.contains(&first_letter) {
        s.push_str("-hay");
    } else {
        let first_letter = s.remove(0);
        let mut suffix = String::from("-");
        suffix.push_str(&(String::from(first_letter)));
        suffix.push_str("ay");
        s.push_str(&suffix);
    }
}