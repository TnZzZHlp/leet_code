use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // 哈希表解法
        let mut map = HashMap::new();

        for (i, v) in nums.iter().enumerate() {
            if let Some(ii) = map.get(&(target - v)) {
                return vec![i as i32, *ii];
            } else {
                map.insert(v, i as i32);
            }
        }

        return vec![];

        // 数组循环写法
        // for i in 0..nums.len() {
        //     for j in i + 1..nums.len() {
        //         if nums[i] + nums[j] == target {
        //             return vec![i as i32, j as i32];
        //         }
        //     }
        // }
        // return vec![];
    }
}

struct Solution {}

fn main() {}
