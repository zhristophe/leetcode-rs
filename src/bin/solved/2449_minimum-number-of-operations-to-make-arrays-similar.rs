/*
 * @lc app=leetcode id=2449 lang=rust
 *
 * [2449] Minimum Number of Operations to Make Arrays Similar
 */

// @lc code=start
impl Solution {
    pub fn make_similar(nums: Vec<i32>, target: Vec<i32>) -> i64 {
        let mut nums = nums;
        let mut target = target;
        nums.sort_unstable();
        target.sort_unstable();
        let divide = |v| {
            let mut v0 = Vec::new();
            let mut v1 = Vec::new();
            for n in v {
                if n % 2 == 0 {
                    v0.push(n);
                } else {
                    v1.push(n);
                }
            }
            (v0, v1)
        };
        let (num0, num1) = divide(nums);
        let (tgt0, tgt1) = divide(target);
        let mut plus0 = 0;
        let mut minus0 = 0;
        for (num, tgt) in num0.iter().zip(tgt0.iter()) {
            let diff = (num - tgt) as i64;
            if diff > 0 {
                plus0 += diff;
            } else {
                minus0 -= diff;
            }
        }
        let mut plus1 = 0;
        let mut minus1 = 0;
        for (num, tgt) in num1.iter().zip(tgt1.iter()) {
            let diff = (num - tgt) as i64;
            if diff > 0 {
                plus1 += diff;
            } else {
                minus1 -= diff;
            }
        }
        let mut ans = 0;
        // dbg!(plus0, minus0, plus1, minus1);
        ans += plus0.min(minus0);
        ans += plus1.min(minus1);
        ans += (plus0 - minus0).abs();
        ans /= 2;

        ans
    }
}
// @lc code=end

struct Solution;
fn main() {
    assert_eq!(
        Solution::make_similar(
            vec![
                70, 1128, 594, 1750, 1712, 176, 1980, 776, 1140, 858, 682, 778, 332, 1886, 762,
                1362
            ],
            vec![
                1244, 1254, 360, 1964, 1038, 322, 2000, 882, 360, 560, 260, 2000, 814, 908, 1904,
                116
            ]
        ),
        483
    );
    assert_eq!(
        Solution::make_similar(
            vec![
                758, 334, 402, 1792, 1112, 1436, 1534, 1702, 1538, 1427, 720, 1424, 114, 1604, 564,
                120, 578
            ],
            vec![
                1670, 216, 1392, 1828, 1104, 464, 678, 1134, 644, 1178, 1150, 1608, 1799, 1156,
                244, 2, 892
            ]
        ),
        645
    );
    assert_eq!(Solution::make_similar(vec![8, 12, 6], vec![2, 14, 10]), 2);
    assert_eq!(Solution::make_similar(vec![1, 2, 5], vec![4, 1, 3]), 1);
    assert_eq!(
        Solution::make_similar(vec![1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1]),
        0
    );
}
