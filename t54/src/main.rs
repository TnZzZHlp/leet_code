impl Solution {
    pub fn spiral_order(mut matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.is_empty() {
            return Vec::new();
        }

        let mut result = Vec::new();

        for i in 0..matrix.len() {
            if i % 2 == 0 {
                result.append(&mut matrix[0]);

                matrix.remove(0);

                for r in &mut matrix {
                    if let Some(f) = r.pop() {
                        result.push(f);
                    }
                }
            } else {
                let mut last = matrix.pop().unwrap();
                last.reverse();
                result.append(&mut last);

                for r in matrix.iter_mut().rev() {
                    if let Some(f) = r.first() {
                        result.push(*f);
                        r.remove(0);
                    }
                }
            }
        }

        result
    }
}

struct Solution {}

fn main() {
    // assert_eq!(
    //     Solution::spiral_order(vec![
    //         vec![1, 2, 3, 4],
    //         vec![5, 6, 7, 8],
    //         vec![9, 10, 11, 12],
    //     ]),
    //     vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
    // );
    assert_eq!(
        Solution::spiral_order(vec![vec![7], vec![9], vec![6]]),
        vec![7, 9, 6]
    )
}
