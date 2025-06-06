/*
 * @lc app=leetcode.cn id=2133 lang=rust
 * @lcpr version=30204
 *
 * [2133] 检查是否每一行每一列都包含全部整数
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::check_valid(vec![vec![1, 2, 3], vec![3, 1, 2], vec![2, 3, 1]]),
        true
    );
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn check_valid(matrix: Vec<Vec<i32>>) -> bool {
        let n = matrix.len();
        for i in 0..n {
            let mut f = vec![false; n + 1];
            for j in 0..n {
                if !f[matrix[i][j] as usize] {
                    f[matrix[i][j] as usize] = true;
                } else {
                    return false;
                }
            }
            f = vec![false; n + 1];
            for j in 0..n {
                if !f[matrix[j][i] as usize] {
                    f[matrix[j][i] as usize] = true;
                } else {
                    return false;
                }
            }
        }
        true
    }
}
// @lc code=end

/*
// @lcpr case=start
// [[1,2,3],[3,1,2],[2,3,1]]\n
// @lcpr case=end

// @lcpr case=start
// [[1,1,1],[1,2,3],[1,2,3]]\n
// @lcpr case=end

 */
