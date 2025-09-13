// Created by bh4bxl at 2025/09/11 12:28
// leetgo: 1.4.15
// https://leetcode.com/problems/binary-tree-maximum-path-sum/

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
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_path_sum = i32::MIN;

        Self::max_gain(&root, &mut max_path_sum);

        max_path_sum
    }

    fn max_gain(node: &Option<Rc<RefCell<TreeNode>>>, max_sum: &mut i32) -> i32 {
        if let Some(n) = node {
            let np = n.borrow();
            let left_gain = Self::max_gain(&np.left, max_sum).max(0);
            let right_gain = Self::max_gain(&np.right, max_sum).max(0);
            let res = np.val.saturating_add(left_gain).saturating_add(right_gain);
            *max_sum = std::cmp::max(*max_sum, res);
            np.val.saturating_add(left_gain.max(right_gain))
        } else {
            i32::MIN
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let root: BinaryTree = deserialize(&read_line()?)?;
    let ans: i32 = Solution::max_path_sum(root.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
