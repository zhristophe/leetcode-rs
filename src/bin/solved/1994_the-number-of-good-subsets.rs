/*
 * @lc app=leetcode.cn id=1994 lang=rust
 * @lcpr version=30204
 *
 * [1994] 好子集的数目
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::number_of_good_subsets(vec![1, 2, 3, 4]), 6);
    assert_eq!(Solution::number_of_good_subsets(vec![4, 2, 3, 15]), 5);
    assert_eq!(Solution::number_of_good_subsets(vec![1, 2, 1]), 4);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn number_of_good_subsets(nums: Vec<i32>) -> i32 {
        let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        let mut dp = vec![0; 1 << primes.len()];
        for num in nums {
            let mut mask = 0;
            let mut f = false;
            for i in 0..primes.len() {
                let p = primes[i];
                if num % (p * p) == 0 {
                    f = true;
                    break;
                }
                if num % p == 0 {
                    mask |= 1 << i;
                }
            }
            if f {
                continue;
            }
            // dbg!(num, mask);
            let mut new_dp = dp.clone();
            new_dp[mask] += 1;
            for i in 0..1 << primes.len() {
                if mask & i == 0 {
                    new_dp[mask | i] += dp[i];
                    new_dp[mask | i] %= 10_0000_0007;
                }
            }
            // dbg!(&new_dp[..8]);
            dp = new_dp;
        }

        dp.iter().skip(1).fold(0, |acc, v| (acc + v) % 10_0000_0007)
    }
}
// @lc code=end

/*
// @lcpr case=start
// [1,2,3,4]\n
// @lcpr case=end

// @lcpr case=start
// [4,2,3,15]\n
// @lcpr case=end

 */
