/*
 * @lc app=leetcode.cn id=1169 lang=rust
 * @lcpr version=30204
 *
 * [1169] 查询无效交易
 */

// @lcpr-template-start
struct Solution;
fn main() {
    let invalid_transactions = |transactions: Vec<&str>| {
        Solution::invalid_transactions(transactions.into_iter().map(|s| s.to_owned()).collect())
    };
    assert_eq!(
        invalid_transactions(vec!["alice,20,800,mtv", "alice,50,100,beijing"]),
        vec!["alice,20,800,mtv", "alice,50,100,beijing"]
    );
    assert_eq!(
        invalid_transactions(vec![
            "alice,20,800,mtv",
            "alice,50,100,mtv",
            "alice,51,100,frankfurt"
        ]),
        vec![
            "alice,20,800,mtv",
            "alice,50,100,mtv",
            "alice,51,100,frankfurt"
        ]
    );
    assert_eq!(
        invalid_transactions(vec![
            "bob,689,1910,barcelona",
            "alex,696,122,bangkok",
            "bob,832,1726,barcelona",
            "bob,820,596,bangkok",
            "chalicefy,217,669,barcelona",
            "bob,175,221,amsterdam"
        ]),
        vec![
            "bob,689,1910,barcelona",
            "bob,832,1726,barcelona",
            "bob,820,596,bangkok"
        ]
    );
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn invalid_transactions(transactions: Vec<String>) -> Vec<String> {
        #[derive(Debug)]
        struct Transaction<'a> {
            name: &'a str,
            time: i32,
            money: i32,
            city: &'a str,
            index: usize,
        }
        let raw_transactions = transactions;
        let mut transactions: Vec<Transaction> = raw_transactions
            .iter()
            .enumerate()
            .map(|(index, s)| {
                let mut iter = s.split(',');
                Transaction {
                    name: iter.next().unwrap(),
                    time: iter.next().unwrap().parse().unwrap(),
                    money: iter.next().unwrap().parse().unwrap(),
                    city: iter.next().unwrap(),
                    index,
                }
            })
            .collect();
        transactions.sort_by_key(|t| t.time);
        // dbg!(&transactions);

        let mut j = 0;
        let mut ans = vec![false; raw_transactions.len()];
        for (i, ti) in transactions.iter().enumerate() {
            if ti.money > 1000 {
                ans[ti.index] = true;
            }
            while j < i && transactions[j].time < ti.time - 60 {
                j += 1;
            }
            for k in j..i {
                let tk = &transactions[k];
                if tk.name == ti.name && tk.city != ti.city {
                    ans[ti.index] = true;
                    ans[tk.index] = true;
                }
            }
        }

        raw_transactions
            .into_iter()
            .enumerate()
            .filter_map(|(index, t)| if ans[index] { Some(t) } else { None })
            .collect()
    }
}
// @lc code=end

/*
// @lcpr case=start
// ["alice,20,800,mtv","alice,50,100,beijing"]\n
// @lcpr case=end

// @lcpr case=start
// ["alice,20,800,mtv","alice,50,1200,mtv"]\n
// @lcpr case=end

// @lcpr case=start
// ["alice,20,800,mtv","bob,50,1200,mtv"]\n
// @lcpr case=end

 */
