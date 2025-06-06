/*
 * @lc app=leetcode.cn id=1239 lang=rust
 * @lcpr version=30204
 *
 * [1239] 串联字符串的最大长度
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::max_length(vec!["un".to_string(), "iq".to_string(), "ue".to_string()]),
        4
    );
    assert_eq!(
        Solution::max_length(vec![
            "ab".to_string(),
            "ba".to_string(),
            "cd".to_string(),
            "dc".to_string()
        ]),
        4
    );
}
// @lcpr-template-end
// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        // 子序列，不需要连续，典型的动态规划问题。
        // 但是记忆空间太大，使用搜索
        let n = arr.len();
        fn solve(
            index: usize,
            chs: i32,
            memo: &mut HashMap<(usize, i32), i32>,
            arr: &[String],
        ) -> i32 {
            if let Some(ans) = memo.get(&(index, chs)) {
                return *ans;
            }
            let mut new_chs = chs;
            let able = (|| {
                for c in arr[index].as_bytes() {
                    let msk = 1 << (c - b'a');
                    if new_chs & msk != 0 {
                        return false;
                    }
                    new_chs |= msk;
                }
                true
            })();
            let mut ans = if index == 0 {
                0
            } else {
                solve(index - 1, chs, memo, arr)
            };
            if able {
                ans = ans.max({
                    let len = arr[index].as_bytes().len() as i32;
                    if index == 0 {
                        len
                    } else {
                        len + solve(index - 1, new_chs, memo, arr)
                    }
                })
            }
            memo.insert((index, chs), ans);

            ans
        }

        solve(n - 1, 0, &mut HashMap::new(), &arr)
    }
}
// @lc code=end

/*
// @lcpr case=start
// ["un","iq","ue"]\n
// @lcpr case=end

// @lcpr case=start
// ["cha","r","act","ers"]\n
// @lcpr case=end

// @lcpr case=start
// ["abcdefghijklmnopqrstuvwxyz"]\n
// @lcpr case=end

 */
