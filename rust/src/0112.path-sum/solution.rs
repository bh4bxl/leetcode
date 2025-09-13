// Created by bh4bxl at 2025/08/30 09:51
// leetgo: 1.4.15
// https://leetcode.com/problems/path-sum/

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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if root.is_none() {
            return false;
        }
        Self::inorder(&root, 0, target_sum)
    }

    fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, sum: i32, target_sum: i32) -> bool {
        if let Some(n) = node {
            let np = n.borrow();
            let val = np.val;
            if np.left.is_none() && np.right.is_none() {
                if sum + val == target_sum {
                    return true;
                } else {
                    return false;
                }
            }
            return Self::inorder(&np.left, sum + val, target_sum)
                || Self::inorder(&np.right, sum + val, target_sum);
        }

        false
    }
}

// @lc code=end

fn main() -> Result<()> {
    let root: BinaryTree = deserialize(&read_line()?)?;
    let target_sum: i32 = deserialize(&read_line()?)?;
    let ans: bool = Solution::has_path_sum(root.into(), target_sum).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
