impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        nums.windows(2).all(|v| v[0] >= v[1]) || nums.windows(2).all(|v| v[0] <= v[1])
    }
}

fn main() {}

struct Solution {}
