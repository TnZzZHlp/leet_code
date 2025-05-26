impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let max = salary.iter().max().unwrap();
        let min = salary.iter().min().unwrap();
        salary
            .iter()
            .filter(|v| *v != max && *v != min)
            .map(|v| *v as f64)
            .sum::<f64>()
            / (salary.len() - 2) as f64
    }
}
struct Solution {}

fn main() {}
