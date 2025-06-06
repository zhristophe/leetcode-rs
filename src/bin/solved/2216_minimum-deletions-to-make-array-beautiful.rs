/*
 * @lc app=leetcode.cn id=2216 lang=rust
 * @lcpr version=30204
 *
 * [2216] 美化数组的最少删除数
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::min_deletion(vec![1, 1, 2, 3, 5]), 1);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn min_deletion(nums: Vec<i32>) -> i32 {
        // 两两一组，当某组相同时，如果前面删除偶数个，必须删其中一个
        // let mut f = vec![0; nums.len()]; // 分别表示删除偶、奇数个的个数
        // 相同的数删哪个都一样
        let n = nums.len();
        let mut ans = 0;
        let mut i = 0;
        while i < n {
            let mut j = i + 1;
            while j < n && nums[j] == nums[i] {
                j += 1;
            }
            ans += j - i - 1;
            i = j + 1;
        }
        if (n - ans) % 2 == 1 {
            ans += 1;
        }

        ans as i32
    }
}
// @lc code=end

/*
// @lcpr case=start
// [1,1,2,3,5]\n
// @lcpr case=end

// @lcpr case=start
// [1,1,2,2,3,3]\n
// @lcpr case=end

 */
