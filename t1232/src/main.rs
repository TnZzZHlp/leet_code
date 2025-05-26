impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        let mut last_slope = None;
        for i in 0..coordinates.len() - 1 {
            let detal_x = coordinates[i + 1][0] - coordinates[i][0];
            let detal_y = coordinates[i + 1][1] - coordinates[i][1];

            let mut slope = 0.0;

            if detal_x == 0 {
                slope = f64::MAX
            } else {
                slope = detal_y as f64 / detal_x as f64;
            }

            if let Some(s) = last_slope {
                if s != slope {
                    return false;
                }
            } else {
                last_slope = Some(slope)
            }
        }

        true
    }
}

struct Solution {}

fn main() {
    assert_eq!(
        Solution::check_straight_line(vec![vec![0, 0], vec![0, 1], vec![0, -1]]),
        true
    )
}
