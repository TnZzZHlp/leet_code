struct Solution {}

impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        if nums.iter().any(|v| *v == 0) {
            0
        } else {
            nums.into_iter()
                .fold(1, |acc, v| if v < 0 { acc * -1 } else { acc })
        }
    }
}

fn main() {
    println!(
        "{}",
        Solution::array_sign(vec![
            41, 65, 14, 80, 20, 10, 55, 58, 24, 56, 28, 86, 96, 10, 3, 84
        ])
    )
}
