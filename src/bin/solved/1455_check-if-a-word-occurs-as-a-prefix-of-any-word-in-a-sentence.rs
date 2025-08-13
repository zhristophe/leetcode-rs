/*
 * @lc app=leetcode.cn id=1455 lang=rust
 * @lcpr version=30204
 *
 * [1455] 检查单词是否为句中其他单词的前缀
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::is_prefix_of_word("i love eating burger".to_string(), "burg".to_string()),
        4
    );
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        let sentence = sentence.as_bytes();
        let search_word = search_word.as_bytes();
        let mut i = 0;
        let mut j;
        let mut ans = 1;
        let n = sentence.len();
        loop {
            j = i;
            while j < n && sentence[j] != b' ' {
                j += 1;
            }
            if sentence[i..j].starts_with(search_word) {
                return ans;
            }
            ans += 1;
            i = j + 1;
            if j >= n {
                break;
            }
        }

        -1
    }
}
// @lc code=end

/*
// @lcpr case=start
// "i love eating burger"\n"burg"\n
// @lcpr case=end

// @lcpr case=start
// "this problem is an easy problem"\n"pro"\n
// @lcpr case=end

// @lcpr case=start
// "i am tired"\n"you"\n
// @lcpr case=end

 */
