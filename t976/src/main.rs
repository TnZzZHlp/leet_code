impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums.reverse();

        for v in 0..nums.len() - 2 {
            if nums[v + 2] + nums[v + 1] > nums[v] {
                return nums[v] + nums[v + 1] + nums[v + 2];
            }
        }

        0
    }
}

struct Solution {}

fn main() {}
