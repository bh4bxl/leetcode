// Created by bh4bxl at 2025/11/03 12:43
// leetgo: 1.4.15
// https://leetcode.com/problems/invert-binary-tree/

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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut res = root.clone();

        fn swap(node: &mut TreeNode) {
            std::mem::swap(&mut node.left, &mut node.right);
        }

        fn inorder(node: &mut Option<Rc<RefCell<TreeNode>>>) {
            if let Some(n) = node {
                let mut np = n.borrow_mut();
                swap(&mut np);
                inorder(&mut np.left);
                inorder(&mut np.right);
            }
        }

        inorder(&mut res);

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let root: BinaryTree = deserialize(&read_line()?)?;
    let ans: BinaryTree = Solution::invert_tree(root.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
