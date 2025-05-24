impl Solution {
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        let mut board = vec![[0, 0, 0], [0, 0, 0], [0, 0, 0]];

        let mut a = true;

        fn win(board: &Vec<[i32; 3]>) -> bool {
            for i in 0..3 {
                // 横向检查
                if board[i][0] == board[i][1] && board[i][1] == board[i][2] && board[i][0] != 0 {
                    return true;
                }

                // 纵向检查
                if board[0][i] == board[1][i] && board[1][i] == board[2][i] && board[0][i] != 0 {
                    return true;
                }
            }

            //斜向检查
            if board[0][0] == board[1][1] && board[1][1] == board[2][2] && board[0][0] != 0 {
                return true;
            }

            if board[2][0] == board[1][1] && board[1][1] == board[0][2] && board[2][0] != 0 {
                return true;
            }

            false
        }

        for move1 in &moves {
            if a {
                board[move1[0] as usize][move1[1] as usize] = 1
            } else {
                board[move1[0] as usize][move1[1] as usize] = -1
            }

            // println!(
            //     "{} {} {}\n{} {} {}\n{} {} {}\n",
            //     board[0][0],
            //     board[0][1],
            //     board[0][2],
            //     board[1][0],
            //     board[1][1],
            //     board[1][2],
            //     board[2][0],
            //     board[2][1],
            //     board[2][2],
            // );

            let win = win(&board);

            if win && a {
                return String::from("A");
            } else if win {
                return String::from("B");
            }

            a = !a
        }

        if moves.len() < 9 {
            return String::from("Pending");
        } else {
            return String::from("Draw");
        }
    }
}

struct Solution {}

fn main() {
    Solution::tictactoe(vec![
        vec![1, 0],
        vec![0, 0],
        vec![0, 2],
        vec![1, 1],
        vec![1, 2],
        vec![0, 1],
    ]);
}
