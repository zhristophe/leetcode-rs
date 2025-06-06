/*
 * @lc app=leetcode.cn id=3452 lang=rust
 * @lcpr version=30204
 *
 * [3452] 好数字之和
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::sum_of_good_numbers(vec![1, 3, 2, 1, 5, 4], 2), 12);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn sum_of_good_numbers(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let n = nums.len();
        for (i, num) in nums.iter().enumerate() {
            if (i as i32 - k < 0 || nums[i - k as usize] < *num)
                && (i as i32 + k >= n as i32 || nums[i + k as usize] < *num)
            {
                ans += num;
            }
        }

        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// [1,3,2,1,5,4]\n2\n
// @lcpr case=end

// @lcpr case=start
// [2,1]\n1\n
// @lcpr case=end

 */
