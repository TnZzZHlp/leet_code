impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut x = 0;
        let mut y = 0;

        moves.chars().for_each(|c| match c {
            'U' => y += 1,
            'D' => y -= 1,
            'L' => x -= 1,
            _ => x += 1,
        });

        x == 0 && y == 0
    }
}

struct Solution {}

fn main() {}
