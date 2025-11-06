// Created by bh4bxl at 2025/11/03 10:21
// leetgo: 1.4.15
// https://leetcode.com/problems/count-complete-tree-nodes/

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
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;

        fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, res: &mut i32) {
            if let Some(n) = node {
                *res += 1;
                let np = n.borrow();
                inorder(&np.left, res);
                inorder(&np.right, res);
            }
        }

        inorder(&root, &mut res);

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let root: BinaryTree = deserialize(&read_line()?)?;
    let ans: i32 = Solution::count_nodes(root.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
