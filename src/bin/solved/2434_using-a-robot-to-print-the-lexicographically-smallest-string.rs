/*
 * @lc app=leetcode.cn id=2434 lang=rust
 * @lcpr version=30204
 *
 * [2434] 使用机器人打印字典序最小的字符串
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::robot_with_string("bac".to_string()),
        "abc".to_string()
    );
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn robot_with_string(s: String) -> String {
        // 字典序最小，意味着必须把最小的字符放到t的最后，并输出
        // 然后，s和t均非空，如果s中含有更小的，继续，否则输出t的末尾
        let s = s.as_bytes();
        let mut t = vec![];
        let mut ans = vec![];
        let mut bt = vec![std::collections::BTreeSet::new(); 26];
        for i in 0..s.len() {
            bt[(s[i] - b'a') as usize].insert(i);
        }
        let mut i = 0; // s的索引
        let mut k = 0u8; // 当前的最小字符
        loop {
            // dbg!((i, k, &s, &t, &ans));
            while k < 26 && bt[k as usize].range(i..).next().is_none() {
                k += 1;
            }
            if k == 26 {
                ans.extend(t.into_iter().rev());
                break;
            }
            while t.last().map(|&c| c <= k + b'a') == Some(true) {
                ans.push(t.pop().unwrap());
            }
            let last_k = *bt[k as usize].last().unwrap();
            for j in i..=last_k {
                if s[j] == k + b'a' {
                    ans.push(s[j]);
                } else {
                    t.push(s[j]);
                }
            }
            i = last_k + 1;
        }

        unsafe { String::from_utf8_unchecked(ans) }
    }
}
// @lc code=end

/*
// @lcpr case=start
// "zza"\n
// @lcpr case=end

// @lcpr case=start
// "bac"\n
// @lcpr case=end

// @lcpr case=start
// "bdda"\n
// @lcpr case=end

 */
