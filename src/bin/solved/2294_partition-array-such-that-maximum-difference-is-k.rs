/*
 * @lc app=leetcode.cn id=2294 lang=rust
 * @lcpr version=30204
 *
 * [2294] 划分数组使最大差为 K
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::partition_array(vec![3, 6, 1, 2, 5], 2), 2)
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn partition_array(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut ans = 0;
        let mut i = 0;
        let n = nums.len();
        while i < n {
            let mut j = i;
            while j < n && nums[j] - nums[i] <= k {
                j += 1;
            }
            ans += 1;
            i = j;
        }
        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// [3,6,1,2,5]\n2\n
// @lcpr case=end

// @lcpr case=start
// [1,2,3]\n1\n
// @lcpr case=end

// @lcpr case=start
// [2,2,4,5]\n0\n
// @lcpr case=end

 */
