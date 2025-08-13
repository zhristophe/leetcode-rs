/*
 * @lc app=leetcode.cn id=846 lang=rust
 * @lcpr version=30204
 *
 * [846] 一手顺子
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::is_n_straight_hand(vec![1, 2, 3, 6, 2, 3, 4, 7, 8], 3),
        true
    );
    assert_eq!(Solution::is_n_straight_hand(vec![1, 1], 2), false);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        let n = hand.len();
        if n % group_size as usize != 0 {
            return false;
        }
        let mut hand = hand;
        hand.sort_unstable();
        let mut indices = std::collections::BTreeSet::from_iter(0..n);
        for _ in 0..n / group_size as usize {
            let used = {
                let mut indices = indices.iter();
                let mut used = vec![*indices.next().unwrap()];
                let mut prev = hand[used[0]];

                for _ in 1..group_size {
                    let mut idx = || {
                        while let Some(idx) = indices.next() {
                            if hand[*idx] > prev {
                                return Some(*idx);
                            }
                        }
                        None
                    };
                    let Some(idx) = idx() else {
                        return false;
                    };
                    if hand[idx] != prev + 1 {
                        return false;
                    }
                    used.push(idx);
                    prev += 1;
                }
                used
            };
            for idx in used {
                indices.remove(&idx);
            }
        }

        true
    }
    #[allow(dead_code)]
    pub fn is_n_straight_hand_slow(hand: Vec<i32>, group_size: i32) -> bool {
        let n = hand.len();
        if n % group_size as usize != 0 {
            return false;
        }
        let mut hand = hand;
        hand.sort_unstable();
        let mut i = 0;
        for _ in 0..n / group_size as usize {
            let mut prev = None;
            let mut j = i;
            for _ in 0..group_size {
                while j < n && hand[j] <= prev.unwrap_or(-1) {
                    j += 1;
                }
                if j == n || prev.map(|old| old + 1 == hand[j]) == Some(false) {
                    return false;
                }
                prev = Some(hand[j]);
                hand[j] = -1;
            }
            // dbg!(&hand);
            i += 1;
        }

        true
    }
}
// @lc code=end

/*
// @lcpr case=start
// [1,2,3,6,2,3,4,7,8]\n3\n
// @lcpr case=end

// @lcpr case=start
// [1,2,3,4,5]\n4\n
// @lcpr case=end

 */
