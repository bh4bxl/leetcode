// Created by bh4bxl at 2025/08/27 16:48
// leetgo: 1.4.15
// https://leetcode.com/problems/minimum-depth-of-binary-tree/

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
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::get_min_depth(&root)
    }

    fn get_min_depth(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(n) = node {
            let np = n.borrow();
            if np.left.is_none() {
                return Self::get_min_depth(&np.right) + 1;
            }
            if np.right.is_none() {
                return Self::get_min_depth(&np.left) + 1;
            }
            return std::cmp::min(
                Self::get_min_depth(&np.left),
                Self::get_min_depth(&np.right),
            ) + 1;
        }

        0
    }
}

// @lc code=end

fn main() -> Result<()> {
    let root: BinaryTree = deserialize(&read_line()?)?;
    let ans: i32 = Solution::min_depth(root.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
