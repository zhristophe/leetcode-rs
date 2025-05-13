/*
 * @lc app=leetcode.cn id=488 lang=rust
 * @lcpr version=30204
 *
 * [488] 祖玛游戏
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::find_min_step("WRRBBW".to_string(), "RB".to_string()),
        -1
    );
    assert_eq!(
        Solution::find_min_step("WWRRBBWW".to_string(), "WRBRW".to_string()),
        2
    );
    assert_eq!(
        Solution::find_min_step("G".to_string(), "GGGGG".to_string()),
        2
    );
    assert_eq!(
        Solution::find_min_step("G".to_string(), "G".to_string()),
        -1
    );
    assert_eq!(
        Solution::find_min_step("RRGGBBYYWWRRGGBB".to_string(), "RGBYW".to_string()),
        -1
    );
    assert_eq!(
        Solution::find_min_step("RRWWRRBBRR".to_string(), "WB".to_string()),
        2
    );
}
// @lcpr-template-end
// @lc code=start
use std::collections::{HashMap, HashSet, VecDeque};
impl Solution {
    pub fn find_min_step(board: String, hand: String) -> i32 {
        // 仅两种填球方式是有意义的：
        // 1. 填在与之相同的连续相同球后面
        // 2. 填在与之不同的连续相同球中间
        macro_rules! get_idx {
            ($color:expr) => {
                match $color {
                    'R' => 0,
                    'Y' => 1,
                    'B' => 2,
                    'G' => 3,
                    'W' => 4,
                    _ => unreachable!(),
                }
            };
        }
        let hand = {
            let mut tmp = [0; 5];
            for c in hand.chars() {
                tmp[get_idx!(c)] += 1;
            }
            tmp
        };
        let board: Vec<u8> = board.chars().map(|ch| get_idx!(ch)).collect();
        let mut clear_memo = HashMap::new();
        fn clear(board: Vec<u8>, clear_memo: &mut HashMap<Vec<u8>, Vec<u8>>) -> Vec<u8> {
            if board.is_empty() {
                return vec![];
            }
            if let Some(res) = clear_memo.get(&board) {
                return res.clone();
            }
            let mut i = 0;
            let mut ball = board[0];
            let mut j = 1;
            while i < board.len() {
                while j < board.len() && board[j] == ball {
                    j += 1;
                }
                if j - i >= 3 {
                    let mut new_board = vec![];
                    new_board.extend_from_slice(&board[..i]);
                    new_board.extend_from_slice(&board[j..]);
                    let ans = clear(new_board, clear_memo);
                    clear_memo.insert(board, ans.clone());
                    return ans;
                }
                if j >= board.len() {
                    break;
                }
                ball = board[j];
                i = j;
                j = i + 1;
            }
            clear_memo.insert(board.clone(), board.clone());
            board
        }
        let mut hash = HashSet::new();
        let mut q = VecDeque::new();
        q.push_back((0, board, hand));
        while let Some((step, board, hand)) = q.pop_front() {
            // dbg!((step, &board, &hand));
            if board.len() == 0 {
                return step;
            }
            for ball in 0..5u8 {
                if hand[ball as usize] == 0 {
                    continue;
                }
                // 准备hand
                let mut hand = hand.clone();
                hand[ball as usize] -= 1;
                // 准备board
                for i in 0..=board.len() {
                    if board.get(i) == Some(&ball) {
                        continue;
                    }
                    let ok = (i == 0 || board.get(i - 1) == Some(&ball) ) // 连续同色球
                     || board.get(i - 1) == board.get(i) // 其它连续同色球中间
                     ;
                    if !ok {
                        continue;
                    }
                    let mut board = board.clone();
                    board.insert(i, ball);
                    let board = clear(board, &mut clear_memo);
                    let entry = (board, hand);
                    if hash.contains(&entry) {
                        continue;
                    }
                    hash.insert(entry.clone());
                    q.push_back((step + 1, entry.0, entry.1));
                }
            }
        }

        -1
    }
}
// @lc code=end

/*
// @lcpr case=start
// "WRRBBW"\n"RB"\n
// @lcpr case=end

// @lcpr case=start
// "WWRRBBWW"\n"WRBRW"\n
// @lcpr case=end

// @lcpr case=start
// "G"\n"GGGGG"\n
// @lcpr case=end

// @lcpr case=start
// "RBYYBBRRB"\n"YRBGB"\n
// @lcpr case=end

 */
