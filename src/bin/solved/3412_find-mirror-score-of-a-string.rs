/*
 * @lc app=leetcode.cn id=3412 lang=rust
 * @lcpr version=30204
 *
 * [3412] 计算字符串的镜像分数
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::calculate_score("aczzx".to_string()), 5)
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn calculate_score(s: String) -> i64 {
        // 用栈维护就行，这里用btree set
        use std::collections::BTreeSet;
        let s = s.as_bytes();
        let mut bts = vec![BTreeSet::new(); 26];
        for (i, c) in s.iter().enumerate() {
            bts[(c - b'a') as usize].insert(i);
        }
        let mut score = 0;
        for i in 0..s.len() {
            if let Some(&j) = bts[(b'z' - s[i]) as usize].range(..i).last() {
                score += i - j;
                bts[(s[i] - b'a') as usize].remove(&i);
                bts[(b'z' - s[i]) as usize].remove(&j);
            }
        }
        score as i64
    }
}
// @lc code=end

/*
// @lcpr case=start
// "aczzx"\n
// @lcpr case=end

// @lcpr case=start
// "abcdef"\n
// @lcpr case=end

 */
