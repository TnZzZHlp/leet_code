impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut res = Vec::new();

        operations.iter().for_each(|s| {
            if let Ok(num) = s.parse::<i32>() {
                res.push(num);
            } else if s == "C" {
                res.pop();
            } else if s == "D" {
                res.push(res.last().unwrap() * 2);
            } else if s == "+" {
                res.push(res[res.len() - 1] + res[res.len() - 2]);
            }
        });

        res.iter().fold(0, |acc, v| (acc + v))
    }
}

struct Solution {}

fn main() {}
