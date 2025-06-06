/*
 * @lc app=leetcode.cn id=2338 lang=rust
 * @lcpr version=30204
 *
 * [2338] 统计理想数组的数目
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::ideal_arrays(1, 5), 5);
    assert_eq!(Solution::ideal_arrays(5, 1), 1);
    assert_eq!(Solution::ideal_arrays(2, 5), 10);
    assert_eq!(Solution::ideal_arrays(2747, 5606), 578548938);
}
// @lcpr-template-end
// @lc code=start
use std::{collections::HashMap, usize};
const MOD: i32 = 1_000_000_007;
impl Solution {
    pub fn ideal_arrays(n: i32, max_value: i32) -> i32 {
        // dp会超时，还得是数论
        // 假设最后一个数是m，相当于把m的因子分配到n个位置上
        // k个球放n个盒子，有C(n+k-1,n-1)种放法
        // 相同的因子最多为log(2, max_value)，小于14

        // 缓存C(n+k-1,n-1)
        let n = n as usize;
        let mut c = vec![vec![1; n + 15]; n + 15];
        for tot in 1..n + 15 {
            for slt in 1..tot {
                c[tot][slt] = (c[tot - 1][slt - 1] + c[tot - 1][slt]) % MOD;
            }
        }

        let mut ans = 0;
        let mut sieve = vec![0; max_value as usize + 1];
        ans += 1; // 末尾为1
        for m in 2..=max_value as usize {
            // 枚举末尾
            // 素数筛
            if sieve[m] == 0 {
                for i in (m..=max_value as usize).step_by(m) {
                    sieve[i] = m;
                }
            }
            let mut tmp = 1;
            let mut x = m;
            while x > 1 {
                let p = sieve[x];
                let mut k = 0;
                while x % p == 0 {
                    x /= p;
                    k += 1;
                }
                if k > 0 {
                    tmp *= c[n + k - 1][n - 1] as i64;
                    tmp %= MOD as i64;
                }
            }
            ans += tmp as i32;
            ans %= MOD;
        }

        ans
    }
    #[allow(dead_code)]
    pub fn ideal_arrays_slow(n: i32, max_value: i32) -> i32 {
        // dp会超时
        fn solve(n: i32, max_value: i32, memo: &mut HashMap<(i32, i32), i32>) -> i32 {
            if n == 1 || max_value <= 1 {
                return max_value;
            }
            if let Some(&ans) = memo.get(&(n, max_value)) {
                return ans;
            }
            let mut ans = 0;
            for i in 1..=max_value {
                // 枚举第一个数
                ans += solve(n - 1, max_value / i, memo);
                ans %= MOD;
            }
            memo.insert((n, max_value), ans);

            ans
        }
        solve(n, max_value, &mut HashMap::new())
    }
}
// @lc code=end

/*
// @lcpr case=start
// 2\n5\n
// @lcpr case=end

// @lcpr case=start
// 5\n3\n
// @lcpr case=end

 */
