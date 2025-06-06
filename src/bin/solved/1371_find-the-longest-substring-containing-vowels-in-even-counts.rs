/*
 * @lc app=leetcode.cn id=1371 lang=rust
 * @lcpr version=30204
 *
 * [1371] 每个元音包含偶数次的最长子字符串
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::find_the_longest_substring(String::from("eleetminicoworoep")),
        13
    )
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn find_the_longest_substring(s: String) -> i32 {
        // 动态规划，记录奇偶
        let s = s.as_bytes();
        let n = s.len();
        let mut f = vec![[i32::MIN; 1 << 5]; n + 1];
        for i in 0..=n {
            f[i][0] = 0; // 什么都没有时是偶数
        }
        let idx = |b: u8| match b {
            b'a' => 0,
            b'e' => 1,
            b'i' => 2,
            b'o' => 3,
            b'u' => 4,
            _ => 5,
        };
        for i in 0..n {
            let idx = idx(s[i]);
            if idx < 5 {
                f[i + 1][1 << idx] = 1; // 仅这个元素
                for j in 0..1 << 5 {
                    f[i + 1][j] = f[i + 1][j].max(f[i][j ^ (1 << idx)] + 1);
                }
            } else {
                for j in 0..1 << 5 {
                    f[i + 1][j] = f[i][j] + 1;
                }
            }
        }

        f.iter().map(|a| a[0]).max().unwrap().clone()
    }
}
// @lc code=end

/*
// @lcpr case=start
// "eleetminicoworoep"\n
// @lcpr case=end

// @lcpr case=start
// "leetcodeisgreat"\n
// @lcpr case=end

// @lcpr case=start
// "bcbcbc"\n
// @lcpr case=end

 */
