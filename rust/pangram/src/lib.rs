pub fn is_pangram(sentence: &'static str) -> bool {
    let mut char_counts = vec![0; 26];

    for c in sentence.chars() {
        if c as u32 <= 122 && c as u32 >= 97 {
            char_counts[(c as u32 - 97) as usize] += 1;
        }
        
        if c as u32 <= 90 && c as u32 >= 65 {
            char_counts[(c as u32 - 65) as usize] += 1;
        }
    }

    for count in char_counts {
        if count == 0 {
            return false;
        }
    }

    true
}