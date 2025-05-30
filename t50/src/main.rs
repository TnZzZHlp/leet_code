impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut result = 1.0;
        let is_negative = n.is_negative();
        let mut n = (n as i64).abs();
        let mut base = x;

        while n > 0 {
            if n & 1 == 1 {
                if is_negative {
                    result /= base;
                } else {
                    result *= base;
                }
            }
            base *= base;
            n = n >> 1;
        }

        result
    }
}

struct Solution {}

fn main() {
    println!("{}", Solution::my_pow(2.00000, -2147483648))
}
