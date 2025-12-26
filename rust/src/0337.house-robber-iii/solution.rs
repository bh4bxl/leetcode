// Created by bh4bxl at 2025/12/22 18:08
// leetgo: 1.4.15
// https://leetcode.com/problems/house-robber-iii/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
            if let Some(n) = node {
                let np = n.borrow();

                let (l_rob, l_not) = dfs(np.left.clone());
                let (r_rob, r_not) = dfs(np.right.clone());

                let rob = np.val + l_not + r_not;
                let not_rob = l_rob.max(l_not) + r_rob.max(r_not);

                (rob, not_rob)
            } else {
                (0, 0)
            }
        }

        let (rob, not_rob) = dfs(root);

        rob.max(not_rob)
    }
}

// @lc code=end

fn main() -> Result<()> {
    let root: BinaryTree = deserialize(&read_line()?)?;
    let ans: i32 = Solution::rob(root.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
