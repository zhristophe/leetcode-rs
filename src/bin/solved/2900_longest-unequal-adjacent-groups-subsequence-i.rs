/*
 * @lc app=leetcode.cn id=2900 lang=rust
 * @lcpr version=30204
 *
 * [2900] 最长相邻不相等子序列 I
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::get_longest_subsequence(
            vec!["e".to_string(), "a".to_string(), "b".to_string()],
            vec![0, 0, 1]
        ),
        vec!["e".to_string(), "b".to_string()]
    );
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let mut cur = groups[0];
        let mut ans = vec![words[0].clone()];
        for (index, word) in words.into_iter().enumerate().skip(1) {
            if groups[index] == cur {
                continue;
            }
            cur = groups[index];
            ans.push(word);
        }

        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// ["e","a","b"]\n[0,0,1]\n
// @lcpr case=end

// @lcpr case=start
// ["a","b","c","d"]\n[1,0,1,1]\n
// @lcpr case=end

 */
