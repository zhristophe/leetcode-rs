/*
 * @lc app=leetcode.cn id=329 lang=rust
 * @lcpr version=30204
 *
 * [329] 矩阵中的最长递增路径
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::longest_increasing_path(vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]]),
        4
    )
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        // 搜索每个位置为起点的最长递增路径
        let m = matrix.len();
        let n = matrix[0].len();
        let mut max_len = vec![vec![0; n]; m];
        let mut queue = std::collections::VecDeque::new();
        for i in 0..m {
            for j in 0..n {
                let cur = matrix[i][j];
                let is_biggest = (i == 0 || cur >= matrix[i - 1][j])
                    && (i == m - 1 || cur >= matrix[i + 1][j])
                    && (j == 0 || cur >= matrix[i][j - 1])
                    && (j == n - 1 || cur >= matrix[i][j + 1]);
                if is_biggest {
                    queue.push_back((i, j));
                    max_len[i][j] = 1;
                }
            }
        }
        while let Some((i, j)) = queue.pop_front() {
            let mut nbrs = vec![];
            if i > 0 {
                nbrs.push((i - 1, j));
            }
            if i < m - 1 {
                nbrs.push((i + 1, j));
            }
            if j > 0 {
                nbrs.push((i, j - 1));
            }
            if j < n - 1 {
                nbrs.push((i, j + 1));
            }
            let cur = matrix[i][j];
            let cur_len = max_len[i][j];
            for (i0, j0) in nbrs {
                if matrix[i0][j0] >= cur {
                    continue;
                }
                if max_len[i0][j0] > cur_len {
                    continue;
                }
                max_len[i0][j0] = cur_len + 1;
                queue.push_back((i0, j0));
            }
        }
        max_len
            .iter()
            .map(|line| line.iter())
            .flatten()
            .max()
            .unwrap()
            .clone()
    }
}
// @lc code=end

/*
// @lcpr case=start
// [[9,9,4],[6,6,8],[2,1,1]]\n
// @lcpr case=end

// @lcpr case=start
// [[3,4,5],[3,2,6],[2,2,1]]\n
// @lcpr case=end

// @lcpr case=start
// [[1]]\n
// @lcpr case=end

 */
