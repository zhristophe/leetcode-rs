/*
 * @lc app=leetcode.cn id=3352 lang=rust
 * @lcpr version=30204
 *
 * [3352] 统计小于 N 的 K 可约简整数
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::count_k_reducible_numbers(String::from("111"), 1),
        3
    );
}
// @lcpr-template-end
// @lc code=start
const MOD: i32 = 1_000_000_007;
impl Solution {
    pub fn count_k_reducible_numbers(s: String, k: i32) -> i32 {
        // 第一感觉：小于 2^n 的 k可约数很好算。更进一步。
        // 取最高位的1，若其取1，继续看下一位，否则，后面的数可以任取
        // 因此先计算小于长度的 k-1 可约数，再计算原问题
        let s = s.as_bytes();
        let n = s.len();
        let k = k as usize;
        let mut f = vec![vec![false; k]; n + 1]; // 只需要k-1可约
        f[1][0] = true; // 1的0-可约为真
        for i in 1..k {
            for j in 0..=n {
                f[j][i] = f[j.count_ones() as usize][i - 1];
            }
        }
        // 再计算组合数
        let mut c = vec![vec![1; n]; n]; // 最大从n-1里取
        for i in 1..n {
            for j in 1..i {
                c[i][j] = (c[i - 1][j - 1] + c[i - 1][j]) % MOD;
            }
        }
        // 计算结果
        let mut ones = 0;
        let mut ans = 0;
        for i in 0..n {
            if s[i] == b'0' {
                continue;
            }
            // 前面所有位和s一致，这一位为0，后面的1位和前面加起来要是k-1可约的
            for j in 0.. {
                // 枚举后面的1的个数
                if j + i + 1 > n {
                    break;
                }
                if f[j + ones][k - 1] {
                    ans += c[n - i - 1][j];
                    ans %= MOD;
                }
            }
            ones += 1;
        }

        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// "111"\n1\n
// @lcpr case=end

// @lcpr case=start
// "1000"\n2\n
// @lcpr case=end

// @lcpr case=start
// "1"\n3\n
// @lcpr case=end

 */
