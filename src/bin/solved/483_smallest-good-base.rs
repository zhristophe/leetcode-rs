/*
 * @lc app=leetcode.cn id=483 lang=rust
 * @lcpr version=30204
 *
 * [483] 最小好进制
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::smallest_good_base("13".to_string()),
        "3".to_string()
    );
    assert_eq!(
        Solution::smallest_good_base("1".to_string() + &"0".repeat(18)),
        "9".repeat(18)
    )
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn smallest_good_base(n: String) -> String {
        // 不难推导 k | 1 - n
        // n-1进制一定可以
        // 最长是2进制64位，最短是n-1进制2位
        let n = n.parse::<u64>().unwrap(); // 最多18位数
        for d in (2..=64).rev() {
            // 二分查找k，计算量log(10^18)
            let mut lo = 2;
            let mut hi = n - 1;
            while lo < hi {
                let k = lo + (hi - lo) / 2;
                let sum = k.pow(d) - 1;
                if sum % (k - 1) == 0 {
                    return k.to_string();
                }
                let m = sum / (k - 1);
                // if m

                if sum < 0 {
                    hi = k;
                } else {
                    lo = k + 1;
                }
            }
        }

        "".to_string()
    }
}
// @lc code=end

/*
// @lcpr case=start
// "13"\n
// @lcpr case=end

// @lcpr case=start
// "4681"\n
// @lcpr case=end

// @lcpr case=start
// "1000000000000000000"\n
// @lcpr case=end

 */
