/*
 * @lc app=leetcode.cn id=1921 lang=rust
 * @lcpr version=30204
 *
 * [1921] 消灭怪物的最大数量
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::eliminate_maximum(vec![1, 3, 4], vec![1, 1, 1]), 3);
    assert_eq!(
        Solution::eliminate_maximum(vec![5, 4, 3, 3, 3], vec![1, 1, 5, 3, 1]),
        1
    );
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
        // 把怪物按照到达时间排序
        let n = dist.len();
        let mut arv = vec![0; n + 1];
        for i in 0..n {
            let t = (dist[i] + speed[i] - 1) / speed[i];
            arv[n.min(t as usize)] += 1;
        }

        let mut free = 0; // 可用的射击数量
        // dbg!(&arv);
        for i in 0..=n {
            free -= arv[i];
            if free < 0 {
                return i as i32;
            }
            free += 1;
        }

        n as i32
    }
}
// @lc code=end

/*
// @lcpr case=start
// [1,3,4]\n[1,1,1]\n
// @lcpr case=end

// @lcpr case=start
// [1,1,2,3]\n[1,1,1,1]\n
// @lcpr case=end

// @lcpr case=start
// [3,2,4]\n[5,3,2]\n
// @lcpr case=end

 */
