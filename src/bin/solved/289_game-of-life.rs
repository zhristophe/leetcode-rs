/*
 * @lc app=leetcode.cn id=289 lang=rust
 * @lcpr version=30204
 *
 * [289] 生命游戏
 */

use leetcode_rs::vec2d;

// @lcpr-template-start
struct Solution;
fn main() {
    let mut input = vec2d![[0, 1, 0], [0, 0, 1], [1, 1, 1], [0, 0, 0]];
    assert_eq!(Solution::game_of_life(&mut input), ());
    assert_eq!(input, vec2d![[0, 0, 0], [1, 0, 1], [0, 1, 1], [0, 1, 0]])
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let mut ans = board.clone();
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                let mut cnt = 0;
                let i0 = i as i64;
                let j0 = j as i64;
                for (i, j) in [
                    (i0 - 1, j0 - 1),
                    (i0 - 1, j0),
                    (i0 - 1, j0 + 1),
                    (i0, j0 - 1),
                    (i0, j0 + 1),
                    (i0 + 1, j0 - 1),
                    (i0 + 1, j0),
                    (i0 + 1, j0 + 1),
                ] {
                    if 0 <= i && i < board.len() as i64 && 0 <= j && j < board[0].len() as i64 {
                        if board[i as usize][j as usize] == 1 {
                            cnt += 1;
                        }
                    }
                }
                match board[i][j] {
                    0 => {
                        if cnt == 3 {
                            ans[i][j] = 1;
                        }
                    }
                    1 => {
                        if cnt < 2 || cnt > 3 {
                            ans[i][j] = 0;
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
        *board = ans.clone();
    }
}
// @lc code=end

/*
// @lcpr case=start
// [[0,1,0],[0,0,1],[1,1,1],[0,0,0]]\n
// @lcpr case=end

// @lcpr case=start
// [[1,1],[1,0]]\n
// @lcpr case=end

 */
