/*
 * @lc app=leetcode id=3016 lang=rust
 *
 * [3016] Minimum Number of Pushes to Type Word II
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut cnt = HashMap::new();
        for c in word.chars() {
            *cnt.entry(c).or_insert(0) += 1;
        }
        let mut ans = 0;
        let mut cnt: Vec<_> = cnt.values().cloned().collect();
        cnt.sort_by(|a, b| b.cmp(a));
        // dbg!(&cnt);
        let mut times = 1;
        loop {
            let start = (times - 1) * 8;
            let end = cnt.len().min(times * 8);
            // dbg!(cnt.len(), start,end);
            // dbg!(&cnt[start..end]);
            ans += cnt[start..end].iter().sum::<i32>() * times as i32;
            // dbg!(ans);
            if end == cnt.len() {
                break;
            }
            times += 1;
        }
        ans
    }
}
// @lc code=end

struct Solution;
fn main() {
    assert_eq!(Solution::minimum_pushes("abcde".to_owned()), 5);
    assert_eq!(Solution::minimum_pushes("xyzxyzxyzxyz".to_owned()), 12);
    assert_eq!(
        Solution::minimum_pushes("aabbccddeeffgghhiiiiii".to_owned()),
        24
    );
}
