/*
 * @lc app=leetcode id=2087 lang=rust
 *
 * [2087] Minimum Cost Homecoming of a Robot in a Grid
 */

// @lc code=start
impl Solution {
    pub fn min_cost(
        start_pos: Vec<i32>,
        home_pos: Vec<i32>,
        row_costs: Vec<i32>,
        col_costs: Vec<i32>,
    ) -> i32 {
        let cost = |costs: Vec<i32>, start, end| {
            let range = if start < end {
                start + 1..end + 1
            } else {
                end..start
            };
            costs[range].iter().sum::<i32>()
        };

        cost(row_costs, start_pos[0] as usize, home_pos[0] as usize)
            + cost(col_costs, start_pos[1] as usize, home_pos[1] as usize)
    }
}
// @lc code=end

struct Solution;
fn main() {
    assert_eq!(
        Solution::min_cost(vec![1, 0], vec![2, 3], vec![5, 4, 3], vec![8, 2, 6, 7]),
        18
    );
}
