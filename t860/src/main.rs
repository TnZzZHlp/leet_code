impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut five = 0;
        let mut ten = 0;

        for amount in bills {
            match amount {
                5 => five += 1,
                10 => ten += 1,
                _ => {}
            }

            match amount - 5 {
                15 => {
                    if ten > 0 && five > 0 {
                        ten -= 1;
                        five -= 1;
                        continue;
                    }

                    if five - 3 >= 0 {
                        five -= 3;
                        continue;
                    } else {
                        return false;
                    }
                }
                10 => {
                    if ten > 0 {
                        ten -= 1;
                        continue;
                    }

                    if five - 2 >= 0 {
                        five -= 2;
                        continue;
                    } else {
                        return false;
                    }
                }
                5 => {
                    if five > 0 {
                        five -= 1;
                        continue;
                    } else {
                        return false;
                    }
                }
                _ => {}
            }
        }

        true
    }
}

struct Solution {}

fn main() {
    assert_eq!(Solution::lemonade_change(vec![5, 5, 10, 10, 20]), false);
}
