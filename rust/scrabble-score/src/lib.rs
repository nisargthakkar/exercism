pub fn score(word: &'static str) -> u32 {
    let char_scores = vec![1, 3, 3, 2, 1, 4, 2, 4, 1, 8, 5, 1, 3, 1, 1, 3, 10, 1, 1, 1, 1, 4, 4, 8, 4, 10];
    let mut counts: Vec<u32> = vec![0; 26];

    for c in word.chars() {
        let char_number = c as u32;
        if char_number <= 122 && char_number >= 97 {
            counts[(char_number - 97) as usize] += char_scores[(char_number - 97) as usize];
        } else if char_number <= 90 && char_number >= 65 {
            counts[(char_number - 65) as usize] += char_scores[(char_number - 65) as usize];
        }
    }

    let mut score = 0;

    for count in counts {
        score += count;
    }

    score
}