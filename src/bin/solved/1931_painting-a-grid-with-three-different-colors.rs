/*
 * @lc app=leetcode.cn id=1931 lang=rust
 * @lcpr version=30204
 *
 * [1931] 用三种不同颜色为网格涂色
 */

// @lcpr-template-start
struct Solution;
fn main() {
    // assert_eq!(Solution::color_the_grid(2, 2), 6);
    assert_eq!(Solution::color_the_grid(1, 2), 6);
}
// @lcpr-template-end
// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn color_the_grid(m: i32, n: i32) -> i32 {
        // 根据上一列的填色情况，为新行填色
        // 用i32代表列情况，每两位表示一格的颜色
        //  3 * 2^4 * (2^5)，缓存空间很大但可接受
        // 枚举全部可能的列
        let mut cols = vec![];
        for col in 0..(1 << (m * 2)) {
            let mut f = true;
            for i in 0..m {
                let c1 = (col >> (i * 2)) & 3;
                let c2 = (col >> (i * 2 + 2)) & 3;
                if c1 == 0 || c1 == c2 {
                    f = false;
                    break;
                }
            }
            if f {
                cols.push(col);
            }
        }
        // 枚举每列可能的下一列
        fn search(col: i32, pos: i32, res: &mut Vec<i32>, prev_col: i32, m: i32) {
            // dbg!((col, pos, prev_col, m));
            if pos == m {
                res.push(col);
                return;
            }
            let mut ok = [true; 4];
            for col in [(prev_col >> (2 * (m - pos - 1))) & 3, col & 3] {
                ok[col as usize] = false;
            }
            for i in 1..=3 {
                if ok[i] {
                    search((col << 2) | i as i32, pos + 1, res, prev_col, m);
                }
            }
        }
        let mut memo = HashMap::new();
        for &col in &cols {
            let mut next_cols = vec![];
            search(0, 0, &mut next_cols, col, m);
            memo.insert(col, next_cols);
        }
        // dbg!(&memo);
        // 递推
        let mut f = vec![0; 1 << (m * 2)];
        for &col in &cols {
            f[col as usize] = 1;
        }
        for _ in 1..n {
            let mut next_f = vec![0; 1 << (m * 2)];
            for &col in &cols {
                for &next_col in &memo[&col] {
                    // dbg!((col, next_col));
                    next_f[next_col as usize] += f[col as usize];
                    next_f[next_col as usize] %= 1_000_000_007;
                }
            }
            f = next_f;
        }

        let mut ans = 0;
        for &col in &cols {
            ans += f[col as usize];
            ans %= 1_000_000_007;
        }

        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// 1\n1\n
// @lcpr case=end

// @lcpr case=start
// 1\n2\n
// @lcpr case=end

// @lcpr case=start
// 5\n5\n
// @lcpr case=end

 */
