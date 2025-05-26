impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        let mut result = 0;
        for i in low..high + 1 {
            if i % 2 != 0 {
                result += 1
            }
        }
        result
    }
}

struct Solution {}

fn main() {}
