impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts
            .iter()
            .max_by_key(|account| account.iter().sum::<i32>())
            .unwrap()
            .iter()
            .sum()
    }
}

struct Solution {}

fn main() {}
