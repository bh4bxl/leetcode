// Created by bh4bxl at 2025/08/21 18:09
// leetgo: 1.4.15
// https://leetcode.com/problems/validate-binary-search-tree/

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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::check(&root, None, None)
    }

    fn check(node: &Option<Rc<RefCell<TreeNode>>>, min: Option<i32>, max: Option<i32>) -> bool {
        if let Some(rc) = node {
            let n = rc.borrow();
            let v = n.val;

            if let Some(lo) = min {
                if v <= lo {
                    return false;
                }
            }
            if let Some(hi) = max {
                if v >= hi {
                    return false;
                }
            }

            Self::check(&n.left, min, Some(v)) && Self::check(&n.right, Some(v), max)
        } else {
            true
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let root: BinaryTree = deserialize(&read_line()?)?;
    let ans: bool = Solution::is_valid_bst(root.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
