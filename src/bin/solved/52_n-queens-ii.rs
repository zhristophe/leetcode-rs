/*
 * @lc app=leetcode.cn id=52 lang=rust
 * @lcpr version=30204
 *
 * [52] N 皇后 II
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::total_n_queens(4), 2);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let n = n as usize;
        let mut lines = vec![0; n];
        let mut ans = 0;
        fn search(lines: &mut [usize], line: usize, n: usize, ans: &mut usize) {
            if line == n {
                *ans += 1;
                return;
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

        ans as i32
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
