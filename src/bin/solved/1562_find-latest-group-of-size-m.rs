/*
 * @lc app=leetcode.cn id=1562 lang=rust
 * @lcpr version=30204
 *
 * [1562] 查找大小为 M 的最新分组
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::find_latest_step(vec![3, 5, 1, 2, 4], 1), 4);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn find_latest_step(arr: Vec<i32>, m: i32) -> i32 {
        // 并查集，或者针对该题可以简化
        let n = arr.len();
        let mut l = vec![0; n + 1];
        let mut r = vec![0; n + 1];
        for i in 1..n {
            l[i] = i;
            r[i] = i;
        }
        let mut ans = -1;
        let mut cnt = 0; // 统计长度为 m 的子数组的个数
        let mut f = vec![false; n + 1]; // 是否是1
        for (index, i) in arr.into_iter().enumerate() {
            let i = i as usize - 1;
            if f[i] {
                continue;
            }
            f[i] = true;
            let mut l0 = i;
            if i > 0 && f[i - 1] {
                l0 = l[i - 1];
                if i - l0 == m as usize {
                    cnt -= 1;
                }
                r[l0] = i;
                l[i] = l0;
            };
            let mut r0 = i;
            if i < n - 1 && f[i + 1] {
                r0 = r[i + 1];
                if r0 - i == m as usize {
                    cnt -= 1;
                }
                r[l0] = r0;
                l[r0] = l0;
            }
            if r0 - l0 + 1 == m as usize {
                cnt += 1;
            }
            // dbg!(cnt);
            // dbg!(&l);
            // dbg!(&r);
            if cnt > 0 {
                ans = index as i32 + 1;
            }
        }

        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// [3,5,1,2,4]\n1\n
// @lcpr case=end

// @lcpr case=start
// [3,1,5,4,2]\n2\n
// @lcpr case=end

// @lcpr case=start
// [1]\n1\n
// @lcpr case=end

// @lcpr case=start
// [2,1]\n2\n
// @lcpr case=end

 */
