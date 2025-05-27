impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let len = if a.len() > b.len() { a.len() } else { b.len() };

        let mut a: Vec<char> = a.chars().collect();
        let mut b: Vec<char> = b.chars().collect();
        a.reverse();
        b.reverse();

        let mut result: Vec<char> = Vec::new();

        for i in 0..len {
            let ai = *a.get(i).unwrap_or(&'0');
            let bi = *b.get(i).unwrap_or(&'0');

            if ai == '1' && bi == '1' {
                // 判断该位置上是否已经有数字
                if let Some(v) = result.get(i) {
                    // 当前为有数字并且为1，说明需要进位并且当前位也为1
                    if *v == '1' {
                        // 所以下一位也为1
                        result.push('1');
                    }
                } else {
                    // 当前位置没有数字说明需要仅为并且当前位为0
                    result.push('0');
                    result.push('1');
                }
            } else if (ai == '1' && bi == '0') || (ai == '0' && bi == '1') {
                // 先判断该位置有没有1，如果有则把该位置改为 0 然后push 1
                if let Some(v) = result.get(i) {
                    if *v == '1' {
                        result[i] = '0';
                        result.push('1');
                    }
                } else {
                    result.push('1');
                }
            } else {
                // 判断该位置有没有数值，如果没有再push 0
                if let None = result.get(i) {
                    result.push('0');
                }
            }
        }

        result.reverse();
        result.iter().collect()
    }
}

struct Solution {}

fn main() {
    assert_eq!(
        Solution::add_binary(String::from("1010"), String::from("1011")),
        String::from("10101")
    );
    assert_eq!(
        Solution::add_binary(String::from("11"), String::from("1")),
        String::from("100")
    );
    assert_eq!(
        Solution::add_binary(String::from("1111"), String::from("1111")),
        String::from("11110")
    );
}
