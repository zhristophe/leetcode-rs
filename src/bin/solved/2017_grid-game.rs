/*
 * @lc app=leetcode id=2017 lang=rust
 *
 * [2017] Grid Game
 */

use std::i64;

use leetcode_rs::vec2d;

// @lc code=start
impl Solution {
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        let grid: Vec<Vec<_>> = grid
            .into_iter()
            .map(|line| line.into_iter().map(|i| i as i64).collect())
            .collect();
        let n = grid[0].len();
        let mut v0 = grid[0][1..].iter().sum::<i64>();
        let mut v1 = 0;
        let mut minv = v0;
        for i in 1..n {
            v0 -= grid[0][i];
            v1 += grid[1][i - 1];
            let new_v = v0.max(v1);
            if new_v < minv {
                minv = new_v;
            }
        }

        minv
    }
}
// @lc code=end

struct Solution;
fn main() {
    // assert_eq!(Solution::grid_game(vec2d![[2, 5, 4], [1, 5, 1]]), 4);
    // assert_eq!(Solution::grid_game(vec2d![[3, 3, 1], [8, 5, 2]]), 4);
    assert_eq!(
        Solution::grid_game(vec2d![
            [20, 3, 20, 17, 2, 12, 15, 17, 4, 15],
            [20, 10, 13, 14, 15, 5, 2, 3, 14, 3]
        ]),
        63
    );
}
