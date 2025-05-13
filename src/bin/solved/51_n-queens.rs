/*
 * @lc app=leetcode.cn id=51 lang=rust
 * @lcpr version=30204
 *
 * [51] N 皇后
 */

use leetcode_rs::vec2d;

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::solve_n_queens(1), vec2d![["Q"]]);
    assert_eq!(
        Solution::solve_n_queens(4),
        vec2d![
            [".Q..", "...Q", "Q...", "..Q."],
            ["..Q.", "Q...", "...Q", ".Q.."]
        ]
    );
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;
        let mut lines = vec![0; n];
        let mut ans = Vec::new();
        fn search(lines: &mut [usize], line: usize, n: usize, ans: &mut Vec<Vec<String>>) {
            if line == n {
                ans.push(
                    lines
                        .iter()
                        .map(|&col| {
                            let mut chars = vec!['.'; n];
                            chars[col] = 'Q';
                            chars.iter().collect()
                        })
                        .collect(),
                );
            }
            let mut choices = vec![true; n];
            for i in 0..line {
                choices[lines[i]] = false;
                if lines[i] >= (line - i) {
                    choices[lines[i] - (line - i)] = false;
                }
                if lines[i] + (line - i) < n {
                    choices[lines[i] + (line - i)] = false;
                }
            }
            for j in 0..n {
                if choices[j] {
                    lines[line] = j;
                    search(lines, line + 1, n, ans);
                }
            }
        }
        search(&mut lines, 0, n, &mut ans);

        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// 4\n
// @lcpr case=end

// @lcpr case=start
// 1\n
// @lcpr case=end

 */
