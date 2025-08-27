// Created by bh4bxl at 2025/08/27 13:43
// leetgo: 1.4.15
// https://leetcode.com/problems/balanced-binary-tree/

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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::check_bal(&root)
    }

    fn check_bal(node: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(n) = node {
            let np = n.borrow();
            let lh = Self::get_height(&np.left);
            let rh = Self::get_height(&np.right);
            let diff = lh as i32 - rh as i32;
            if diff.abs() > 1 {
                return false;
            }

            return Self::check_bal(&np.left) && Self::check_bal(&np.right);
        }

        true
    }

    fn get_height(node: &Option<Rc<RefCell<TreeNode>>>) -> usize {
        if let Some(n) = node {
            let np = n.borrow();
            return std::cmp::max(Self::get_height(&np.left), Self::get_height(&np.right)) + 1;
        }

        0
    }
}

// @lc code=end

fn main() -> Result<()> {
    let root: BinaryTree = deserialize(&read_line()?)?;
    let ans: bool = Solution::is_balanced(root.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
