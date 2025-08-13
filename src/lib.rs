#[macro_export]
macro_rules! vec2d {
    [$([$($x:expr),* $(,)?]),* $(,)?] => {
        vec![
            $(vec![$($x),*]),*
        ]
    };
}

#[macro_export]
macro_rules! grid {
    [$($tokens:tt)*] => {
        vec2d![$($tokens)*]
    };
}

#[macro_export]
macro_rules! str_vec {
    ($($element:expr),* $(,)?) => {
        vec![
            $($element.to_string()),*
        ]
    };
}

#[macro_export]
macro_rules! tree {
    [$($x:tt),+ $(,)?] => {{
        let nodes = vec![
            $(match stringify!($x) {
                "null" => None,
                val => Some(Rc::new(RefCell::new(TreeNode::new(val.parse().unwrap()))))
            },)*
        ];
        let mut q = std::collections::VecDeque::new();
        let root = nodes[0].clone();
        q.push_back(root.clone());
        for i in (1..nodes.len()).step_by(2) {
            let mut n = None;
            while n.is_none() {
                n = q.pop_front().unwrap();
            }
            let n = n.unwrap();
            let mut n = n.borrow_mut();
            n.left = nodes[i].clone();
            q.push_back(n.left.clone());
            if i + 1 < nodes.len() {
                n.right = nodes[i + 1].clone();
                q.push_back(n.right.clone());
            }
        }

        root
    }};
}

#[macro_export]
macro_rules! list {
    [$($x:expr),+ $(,)?] => {{
        let vals = vec![$($x),+];
        let mut head = Box::new(ListNode::new(vals[0]));
        let mut cur = &mut head;
        for &x in &vals[1..] {
            cur.next = Some(Box::new(ListNode::new(x)));
            cur = cur.next.as_mut().unwrap();
        }

        Some(head)
    }};
}
