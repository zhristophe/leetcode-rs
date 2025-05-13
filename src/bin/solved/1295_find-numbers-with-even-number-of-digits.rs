/*
 * @lc app=leetcode.cn id=1295 lang=rust
 * @lcpr version=30204
 *
 * [1295] 统计位数为偶数的数字
 */

// @lcpr-template-start

// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for mut num in nums {
            let mut cnt = 0;
            while num > 0 {
                num /= 10;
                cnt += 1;
            }
            if cnt % 2 == 0 && cnt != 0 {
                ans += 1
            }
        }
        ans
    }
}
// @lc code=end

struct Solution;
fn main() {
    assert_eq!(Solution::find_numbers(vec![12, 345, 2, 6, 7896]), 2);
}

/*
// @lcpr case=start
// [12,345,2,6,7896]\n
// @lcpr case=end

// @lcpr case=start
// [555,901,482,1771]\n
// @lcpr case=end

 */
