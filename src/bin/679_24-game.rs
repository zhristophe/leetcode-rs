/*
 * @lc app=leetcode.cn id=679 lang=rust
 * @lcpr version=30204
 *
 * [679] 24 点游戏
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::judge_point24(vec![4, 1, 8, 7]), true);
    assert_eq!(Solution::judge_point24(vec![1, 2, 1, 2]), false);
    assert_eq!(Solution::judge_point24(vec![6, 6, 6, 6]), true);
    assert_eq!(Solution::judge_point24(vec![3, 4, 3, 4]), true);
    assert_eq!(Solution::judge_point24(vec![8, 1, 2, 3]), true);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn judge_point24(cards: Vec<i32>) -> bool {
        // 数据范围小，完全枚举即可
        fn solve(cards: Vec<(i32, i32)>) -> bool {
            if cards.len() == 1 {
                return cards[0].0 == cards[0].1 * 24;
            }
            // dbg!(&cards);
            for i in 0..cards.len() {
                for j in i + 1..cards.len() {
                    let mut new_cards = vec![];
                    for k in 0..cards.len() {
                        if k != i && k != j {
                            new_cards.push(cards[k]);
                        }
                    }
                    let c1 = cards[i];
                    let c2 = cards[j];
                    for new_card in [
                        (c1.0 * c2.1 + c2.0 * c1.1, c1.1 * c2.1),
                        (c1.0 * c2.0, c1.1 * c2.1),
                        (c1.0 * c2.1 - c2.0 * c1.1, c1.1 * c2.1),
                        (-c1.0 * c2.1 + c2.0 * c1.1, c1.1 * c2.1),
                        (c1.0 * c2.1, c1.1 * c2.0),
                        (c2.0 * c1.1, c2.1 * c1.0),
                    ] {
                        if new_card.1 == 0 {
                            continue;
                        }
                        let mut new_cards = new_cards.clone();
                        new_cards.push(new_card);
                        if solve(new_cards) {
                            return true;
                        }
                    }
                }
            }
            false
        }

        solve(cards.into_iter().map(|c| (c, 1)).collect())
    }
}
// @lc code=end

/*
// @lcpr case=start
// [4, 1, 8, 7]\n
// @lcpr case=end

// @lcpr case=start
// [1, 2, 1, 2]\n
// @lcpr case=end

 */
