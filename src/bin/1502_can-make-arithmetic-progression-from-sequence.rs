/*
 * @lc app=leetcode.cn id=1502 lang=rust
 * @lcpr version=30204
 *
 * [1502] 判断能否形成等差数列
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::can_make_arithmetic_progression(vec![3, 5, 1]),
        true
    );
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
        let mut arr = arr;
        arr.sort_unstable();
        for i in 1..arr.len() {
            if arr[i] - arr[i - 1] != arr[1] - arr[0] {
                return false;
            }
        }
        true
    }
}
// @lc code=end

/*
// @lcpr case=start
// [3,5,1]\n
// @lcpr case=end

// @lcpr case=start
// [1,2,4]\n
// @lcpr case=end

 */
