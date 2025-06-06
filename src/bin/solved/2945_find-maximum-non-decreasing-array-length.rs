/*
 * @lc app=leetcode.cn id=2945 lang=rust
 * @lcpr version=30204
 *
 * [2945] 找到最大非递减数组的长度
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::find_maximum_length(vec![1, 2, 3, 4, 5]), 5);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn find_maximum_length(nums: Vec<i32>) -> i32 {
        // 似乎只能搜索
        // 复杂度算不出来，摆烂了
        fn solve(n: usize, mut sum: i64, nums: &[i32]) -> i32 {
            for i in (0..n).rev() {}

            if n == 1 {
                return if nums[0] <= maxn { 1 } else { -1 };
            }
            let mut sum = nums[n - 1];
            let mut ans = -1;
            for i in (0..n - 1).rev() {
                if ans >= i as i32 {
                    break;
                }
                let res = solve(i, sum, nums);
                dbg!((i, sum, res));
                if res != -1 {
                    ans = ans.max(res + 1);
                }
                sum += nums[i];
            }
            ans
        }
        solve(nums.len(), i32::MAX, &nums)
    }
}
// @lc code=end

/*
// @lcpr case=start
// [5,2,2]\n
// @lcpr case=end

// @lcpr case=start
// [1,2,3,4]\n
// @lcpr case=end

// @lcpr case=start
// [4,3,2,6]\n
// @lcpr case=end

 */
