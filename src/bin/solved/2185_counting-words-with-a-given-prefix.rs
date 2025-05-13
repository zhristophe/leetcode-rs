/*
 * @lc app=leetcode id=2185 lang=rust
 *
 * [2185] Counting Words With a Given Prefix
 */

// @lc code=start
impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        words.iter().filter(|word| word.starts_with(&pref)).count() as i32
    }
}
// @lc code=end

struct Solution;
fn main() {
    assert_eq!(
        Solution::prefix_count(
            vec![
                "pay".to_owned(),
                "attention".to_owned(),
                "practice".to_owned(),
                "attend".to_owned()
            ],
            "at".to_owned()
        ),
        2
    );
}
