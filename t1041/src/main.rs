impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        enum Direction {
            North,
            East,
            South,
            West,
        }

        let mut x = 0;
        let mut y = 0;

        let directions = vec![
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ];

        // 当前面朝方向
        let mut cur_direction_index = 0;

        for _ in 0..4 {
            instructions.chars().for_each(|char| match char {
                'G' => match directions[cur_direction_index] {
                    Direction::East => x += 1,
                    Direction::West => x -= 1,
                    Direction::South => y -= 1,
                    Direction::North => y += 1,
                },
                'R' => cur_direction_index = (cur_direction_index + 1) % 4,
                'L' => cur_direction_index = (cur_direction_index + 3) % 4,
                _ => {}
            });

            if cur_direction_index == 0 {
                break;
            }
        }

        (x, y) == (0, 0)
    }
}

struct Solution {}

fn main() {
    Solution::is_robot_bounded(String::from("GGLLGG"));
}
