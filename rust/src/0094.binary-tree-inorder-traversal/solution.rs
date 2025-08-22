// Created by bh4bxl at 2025/08/19 21:03
// leetgo: 1.4.15
// https://leetcode.com/problems/binary-tree-inorder-traversal/

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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![];
        let mut curr = root;

        while curr.is_some() || !stack.is_empty() {
            while let Some(node) = curr {
                curr = node.borrow().left.clone();
                stack.push(node);
            }
            if let Some(node) = stack.pop() {
                res.push(node.borrow().val);
                curr = node.borrow().right.clone();
            }
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let root: BinaryTree = deserialize(&read_line()?)?;
    let ans: Vec<i32> = Solution::inorder_traversal(root.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
