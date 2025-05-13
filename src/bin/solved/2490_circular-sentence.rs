/*
 * @lc app=leetcode.cn id=2490 lang=rust
 * @lcpr version=30204
 *
 * [2490] 回环句
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::is_circular_sentence("leetcode exercises sound delightful".to_string()),
        true
    );
    assert_eq!(
        Solution::is_circular_sentence("leetcode  exercises sound delightful".to_string()),
        false
    );
    assert_eq!(
        Solution::is_circular_sentence(" leetcode exercises sound delightful".to_string()),
        false
    );
    assert_eq!(Solution::is_circular_sentence("a".to_string()), true);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        assert!(sentence.len() > 0);
        let chars = sentence.chars().collect::<Vec<_>>();
        if !matches!(chars.first(), Some('a'..='z' | 'A'..='Z'))
            || !matches!(chars.last(), Some('a'..='z' | 'A'..='Z'))
        {
            return false;
        }
        if chars
            .iter()
            .any(|ch| !matches!(ch, 'a'..='z' | 'A'..='Z' | ' '))
        {
            return false;
        }
        if chars
            .windows(2)
            .any(|chs| chs[0] == chs[1] && chs[0] == ' ')
        {
            return false;
        }
        let words = sentence.split(' ').collect::<Vec<_>>();
        if words.len() <= 0 {
            return false;
        }
        words
            .windows(2)
            .all(|win| win[0].chars().last() == win[1].chars().next())
            && words.first().unwrap().chars().next() == words.last().unwrap().chars().last()
    }
}
// @lc code=end

/*
// @lcpr case=start
// "leetcode exercises sound delightful"\n
// @lcpr case=end

// @lcpr case=start
// "eetcode"\n
// @lcpr case=end

// @lcpr case=start
// "Leetcode is cool"\n
// @lcpr case=end

 */
