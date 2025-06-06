/*
 * @lc app=leetcode.cn id=887 lang=rust
 * @lcpr version=30204
 *
 * [887] 鸡蛋掉落
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::super_egg_drop(1, 2), 2);
    assert_eq!(Solution::super_egg_drop(2, 6), 3);
    assert_eq!(Solution::super_egg_drop(3, 14), 4);
    assert_eq!(Solution::super_egg_drop(1, 14), 14);
    assert_eq!(Solution::super_egg_drop(2, 1), 1);
    assert_eq!(Solution::super_egg_drop(2, 2), 2);
}
// @lcpr-template-end
// @lc code=start
use std::{collections::HashMap, i32};
impl Solution {
    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        // 经典递推，但是问的是操作次数
        // 1枚鸡蛋，必须从1楼开始试，n次；
        // 2枚鸡蛋，第一个从m楼扔下，
        // 如果碎了，剩一枚，需要一个个试，f(m-1,1)次
        // 如果没碎，相当于原问题的n变为n-m,f(n-m,2)次
        // 由此得到递推公式，但是需要枚举m
        // 利用单调性二分查找
        fn solve(k: i32, n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
            if k == 1 || n <= 1 {
                return n;
            }
            if let Some(ans) = memo.get(&(n * 100 + k)) {
                return *ans;
            }
            let (mut l, mut r) = (1, n);
            while l < r {
                let m = (l + r + 1) / 2;
                let a1 = solve(k - 1, m - 1, memo);
                let a2 = solve(k, n - m, memo);
                if a1 > a2 {
                    r = m - 1;
                } else {
                    l = m;
                }
            }
            let a1 = solve(k - 1, l - 1, memo).max(solve(k, n - l, memo));
            let a2 = solve(k - 1, l, memo).max(solve(k, n - l - 1, memo));
            let ans = a1.min(a2) + 1;
            memo.insert(n * 100 + k, ans);

            ans
        }

        solve(k, n, &mut HashMap::new())
    }
}
// @lc code=end

/*
// @lcpr case=start
// 1\n2\n
// @lcpr case=end

// @lcpr case=start
// 2\n6\n
// @lcpr case=end

// @lcpr case=start
// 3\n14\n
// @lcpr case=end

 */
