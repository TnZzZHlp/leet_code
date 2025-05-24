impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.split(" ")
            .collect::<Vec<&str>>()
            .iter()
            .filter(|x| !x.is_empty())
            .last()
            .unwrap()
            .len() as i32
    }
}

struct Solution {}

fn main() {}
