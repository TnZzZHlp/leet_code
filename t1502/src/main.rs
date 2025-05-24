struct Solution {}

impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        if arr.len() < 2 {
            return false;
        };

        arr.sort();

        let target = arr[1] - arr[0];

        for i in 2..arr.len() {
            if arr[i] - arr[i - 1] != target {
                return false;
            }
        }

        true
    }
}

fn main() {
    assert!(Solution::can_make_arithmetic_progression(vec![3, 5, 1]))
}
