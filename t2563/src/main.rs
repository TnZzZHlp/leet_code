impl Solution {
    pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        let mut count = 0;

        // 先对数组排序
        nums.sort_unstable();

        for j in 0..nums.len() {
            // 注意要在 [0, j-1] 中二分，因为题目要求两个下标 i < j
            let l = nums[..j].partition_point(|&x| x < lower - nums[j]);
            let r = nums[..j].partition_point(|&x| x <= upper - nums[j]);
            count += r - l;
        }

        count as _
    }
}

struct Solution {}

fn main() {
    assert_eq!(Solution::count_fair_pairs(vec![0, 1, 4, 4, 5, 7], 3, 6), 6);
    assert_eq!(Solution::count_fair_pairs(vec![1, 7, 9, 2, 5], 11, 11), 1);
}
