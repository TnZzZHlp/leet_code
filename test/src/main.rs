impl Solution {
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        let mut result = Vec::new();
        words.iter().enumerate().for_each(|(i, word)| {
            if word.contains(x) {
                result.push(i as i32);
            }
        });

        result
    }
}

struct Solution {}

fn main() {}
