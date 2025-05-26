impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let mut sum = 0;
        let mut index = 0;
        let len = mat.len();
        if len % 2 != 0 {
            // 非中心的
            for i in 0..len {
                if i == len / 2 {
                    continue;
                }

                if i < len / 2 {
                    sum += mat[i][index];
                    sum += mat[i][len - index - 1];
                    index += 1;
                } else {
                    index -= 1;
                    sum += mat[i][index];
                    sum += mat[i][len - index - 1];
                }
            }
            // 获取中心值
            sum += mat[len / 2][len / 2]
        } else {
            for i in 0..len {
                if i < len / 2 {
                    sum += mat[i][index];
                    sum += mat[i][len - index - 1];
                    index += 1;
                } else {
                    index -= 1;
                    sum += mat[i][index];
                    sum += mat[i][len - index - 1];
                }
            }
        }
        sum
    }
}

struct Solution {}

fn main() {
    // assert_eq!(
    //     Solution::diagonal_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
    //     25
    // );
    // assert_eq!(
    //     Solution::diagonal_sum(vec![
    //         vec![1, 1, 1, 1],
    //         vec![1, 1, 1, 1],
    //         vec![1, 1, 1, 1],
    //         vec![1, 1, 1, 1]
    //     ]),
    //     8
    // );
    // assert_eq!(
    //     Solution::diagonal_sum(vec![
    //         vec![7, 3, 1, 9],
    //         vec![3, 4, 6, 9],
    //         vec![6, 9, 6, 6],
    //         vec![9, 5, 8, 5]
    //     ]),
    //     55
    // );
    assert_eq!(
        Solution::diagonal_sum(vec![
            vec![7, 9, 8, 6, 3],
            vec![3, 9, 4, 5, 2],
            vec![8, 1, 10, 4, 10],
            vec![9, 5, 10, 9, 6],
            vec![7, 2, 4, 10, 8]
        ]),
        63
    );
}
