/*
 * @lc app=leetcode.cn id=338 lang=rust
 * @lcpr version=30204
 *
 * [338] 比特位计数
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::count_bits(2), vec![0, 1, 1]);
    assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut ans = vec![0; n as usize + 1];
        for i in 0..=n {
            let cnt = {
                let mut cnt = 0;
                let mut i = i;
                while i > 0 {
                    cnt += i & 1;
                    i >>= 1;
                }
                cnt
            };
            ans[i as usize] = cnt;
        }

        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// 2\n
// @lcpr case=end

// @lcpr case=start
// 5\n
// @lcpr case=end

 */
