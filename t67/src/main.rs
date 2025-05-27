use std::result;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let len = if a.len() > b.len() { a.len() } else { b.len() };

        let mut a: Vec<char> = a.chars().collect();
        let mut b: Vec<char> = b.chars().collect();
        a.reverse();
        b.reverse();

        let mut result: Vec<char> = Vec::new();

        for i in 0..len {
            if a[i] == '1' && b[i] == '1' {
                if let Some(v) = result.get(i) {
                    if *v == '1' {
                        result.push('1');
                        result.push('1');
                    } else {
                        result.push('0');
                        result.push('1');
                    }
                } else if (a[i] == '1' && b[i] == '0') || (a[i] == '0' && b[i] == '1') {
                    // 先判断该位置有没有1，如果有则把该位置改为0然后push1
                    if let Some(v) = result.get(i) {
                        if *v == '1' {
                            result[i] = '0';
                            result.push('1');
                        } else {
                            result.push('1');
                        }
                    }
                } else {
                    //
                }
            }
        }
        todo!()
    }
}

struct Solution {}

fn main() {
    assert_eq!(
        Solution::add_binary(String::from("1010"), String::from("1011")),
        String::from("10101")
    );
}
