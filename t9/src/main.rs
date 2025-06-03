impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let x = x.to_string();
        let x_s = x.to_string().chars().into_iter().rev().collect::<String>();
        x_s == x
    }
}

struct Solution {}

fn main() {}
