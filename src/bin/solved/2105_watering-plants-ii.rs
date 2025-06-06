/*
 * @lc app=leetcode.cn id=2105 lang=rust
 * @lcpr version=30204
 *
 * [2105] 给植物浇水 II
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::minimum_refill(vec![2, 2, 3, 3], 5, 5), 1);
    assert_eq!(Solution::minimum_refill(vec![2, 2, 3, 3], 3, 4), 2);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn minimum_refill(plants: Vec<i32>, capacity_a: i32, capacity_b: i32) -> i32 {
        let n = plants.len();
        let mut i = 0;
        let mut j = n - 1;
        let mut a = capacity_a;
        let mut b = capacity_b;
        let mut ans = 0;
        macro_rules! check {
            ($v:expr, $x:expr, $capacity:expr) => {
                if $x < $v {
                    $x = $capacity - $v;
                    ans += 1;
                } else {
                    $x -= $v;
                }
            };
        }
        while i <= j {
            if i == j {
                if a >= b {
                    check!(plants[i], a, capacity_a);
                    let _ = a;
                } else {
                    check!(plants[j], b, capacity_b);
                    let _ = b;
                }
                break;
            }
            check!(plants[i], a, capacity_a);
            check!(plants[j], b, capacity_b);
            i += 1;
            j -= 1;
        }

        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// [2,2,3,3]\n5\n5\n
// @lcpr case=end

// @lcpr case=start
// [2,2,3,3]\n3\n4\n
// @lcpr case=end

// @lcpr case=start
// [5]\n10\n8\n
// @lcpr case=end

 */
