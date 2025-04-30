/*
 * @lc app=leetcode id=3029 lang=rust
 *
 * [3029] Minimum Time to Revert Word to Initial State I
 */

// @lc code=start
impl Solution {
    pub fn minimum_time_to_initial_state(word: String, k: i32) -> i32 {
        for i in 1.. {
            let off = (k * i) as usize;
            if off >= word.len() {
                return i;
            }
            // println!("{} {} {}", off, &word[off..], &word[..word.len() - off]);
            if word[off..] == word[..word.len() - off] {
                return i;
            }
        }
        unreachable!()
    }
}
// @lc code=end

struct Solution;
fn main() {
    assert_eq!(
        Solution::minimum_time_to_initial_state("abacaba".to_string(), 3),
        2
    );
    assert_eq!(
        Solution::minimum_time_to_initial_state("abacaba".to_string(), 4),
        1
    );
    assert_eq!(
        Solution::minimum_time_to_initial_state("abcbabcd".to_string(), 2),
        4
    );
}
