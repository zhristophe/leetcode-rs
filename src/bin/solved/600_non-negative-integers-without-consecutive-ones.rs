/*
 * @lc app=leetcode.cn id=600 lang=rust
 * @lcpr version=30204
 *
 * [600] 不含连续1的非负整数
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::find_integers(0), 1);
    assert_eq!(Solution::find_integers(1), 2);
    assert_eq!(Solution::find_integers(2), 3);
    assert_eq!(Solution::find_integers(3), 3);
    assert_eq!(Solution::find_integers(4), 4);
    assert_eq!(Solution::find_integers(5), 5);
    assert_eq!(Solution::find_integers(6), 5);
    assert_eq!(Solution::find_integers(7), 5);
    assert_eq!(Solution::find_integers(8), 6);
    assert_eq!(Solution::find_integers(9), 7);
    assert_eq!(Solution::find_integers(10), 8);
    assert_eq!(Solution::find_integers(11), 8);
    assert_eq!(Solution::find_integers(12), 8);
    assert_eq!(Solution::find_integers(13), 8);
    assert_eq!(Solution::find_integers(14), 8);
    assert_eq!(Solution::find_integers(15), 8);
    assert_eq!(Solution::find_integers(16), 9);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn find_integers(n: i32) -> i32 {
        // 记d位数满足条件的为f(d)
        // 则f(d) = f(d - 1) + f(d - 2)
        let mut f = vec![0; 32];
        f[0] = 1;
        f[1] = 2;
        for i in 2..32 {
            f[i] = f[i - 1] + f[i - 2];
        }
        let mut ans = 0;
        let mut n = n;
        loop {
            if n <= 2 {
                return ans + n + 1;
            } else if n == 3 {
                return ans + 3;
            }
            let mut digits = 0;
            let mut m = n;
            while m > 0 {
                digits += 1;
                m >>= 1;
            }
            ans += f[digits - 1];
            if n & (1 << (digits - 2)) != 0 {
                return ans + f[digits - 2];
            }
            let mut next = n;
            next &= !(1 << (digits - 1));
            n = next;
        }
    }
}
// @lc code=end

/*
// @lcpr case=start
// 5\n
// @lcpr case=end

// @lcpr case=start
// 1\n
// @lcpr case=end

// @lcpr case=start
// 2\n
// @lcpr case=end

 */
