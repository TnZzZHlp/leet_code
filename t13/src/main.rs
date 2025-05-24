impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        use std::collections::HashMap;

        let map = HashMap::from([
            (b'I', 1),
            (b'V', 5),
            (b'X', 10),
            (b'L', 50),
            (b'C', 100),
            (b'D', 500),
            (b'M', 1000),
        ]);

        let origin: Vec<char> = s.chars().collect();

        let mut target = 0;

        origin.windows(2).for_each(|v| {
            let front = map[&(v[0] as u8)];
            let back = map[&(v[1] as u8)];

            if front < back {
                target -= front;
            } else {
                target += front;
            }
        });

        target + map[&(*origin.last().unwrap() as u8)]
    }
}

struct Solution {}

fn main() {
    assert_eq!(Solution::roman_to_int(String::from("XXVII")), 27);
    assert_eq!(Solution::roman_to_int(String::from("MCMXCIV")), 1994);
    assert_eq!(Solution::roman_to_int(String::from("LVIII")), 58)
}
