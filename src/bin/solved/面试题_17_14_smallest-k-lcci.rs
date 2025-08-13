/*
 * @lc app=leetcode.cn id=面试题 17.14 lang=rust
 * @lcpr version=30204
 *
 * [面试题 17.14] 最小K个数
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::smallest_k(vec![1, 3, 5, 7, 2, 4, 6, 8], 4),
        vec![1, 2, 3, 4]
    );
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn smallest_k(arr: Vec<i32>, k: i32) -> Vec<i32> {
        // 快速选择算法优化
        let mut arr = arr;
        arr.sort_unstable();

        arr[0..k as usize].to_vec()
    }
}
// @lc code=end

/*
// @lcpr case=start
// [1,3,5,7,2,4,6,8]\n4\n
// @lcpr case=end

 */
