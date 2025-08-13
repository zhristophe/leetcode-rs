/*
 * @lc app=leetcode.cn id=782 lang=rust
 * @lcpr version=30204
 *
 * [782] 变为棋盘
 */

use core::borrow;

// @lcpr-template-start
struct Solution;
fn main() {}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn moves_to_chessboard(board: Vec<Vec<i32>>) -> i32 {
        // 所有0的横纵坐标和（差）为偶数（奇数），1则相反，那么成为棋盘
        // 交换i j行时，如果i j奇偶一样，不影响整体优化目标
        // 交换两次操作，无论操作是操作行还是列，不影响结果
        // 贪心法？

        fn solve(board: &Vec<Vec<i32>>) -> i32 {
            0
        }

        let ans = solve(&board);
        let mut board = board;
        let n = board.len();
        for i in 0..n {
            for j in 0..n {
                board[i][j] = 1 - board[i][j];
            }
        }
        let ans = ans.max(solve(&board));

        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// [[0,1,1,0],[0,1,1,0],[1,0,0,1],[1,0,0,1]]\n
// @lcpr case=end

// @lcpr case=start
// [[0, 1], [1, 0]]\n
// @lcpr case=end

// @lcpr case=start
// [[1, 0], [1, 0]]\n
// @lcpr case=end

 */
