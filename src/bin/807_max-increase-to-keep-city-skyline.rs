/*
 * @lc app=leetcode id=807 lang=rust
 *
 * [807] Max Increase to Keep City Skyline
 */

use leetcode_rs::vec2d;

// @lc code=start
impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let n_row = grid.len();
        let n_col = grid[0].len();
        let mut row_max = vec![0; n_row];
        let mut col_max = vec![0; n_col];

        for i in 0..n_row {
            row_max[i] = *grid[i].iter().max().unwrap();
        }
        for j in 0..n_col {
            col_max[j] = (0..n_row).map(|i| grid[i][j]).max().unwrap();
        }
        let mut ans = 0;
        for i in 0..n_row {
            for j in 0..n_col {
                ans += row_max[i].min(col_max[j]) - grid[i][j];
            }
        }
        ans
    }
}
// @lc code=end

struct Solution;
fn main() {
    assert_eq!(
        Solution::max_increase_keeping_skyline(vec2d![
            [3, 0, 8, 4],
            [2, 4, 5, 7],
            [9, 2, 6, 3],
            [0, 3, 1, 0]
        ]),
        35
    );
}
