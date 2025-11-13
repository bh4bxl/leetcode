// Created by bh4bxl at 2025/11/11 16:32
// leetgo: 1.4.15
// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/

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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut node = root;
        let p_val = p.as_ref().unwrap().borrow().val;
        let q_val = q.as_ref().unwrap().borrow().val;

        while let Some(curr) = node.clone() {
            let val = curr.borrow().val;
            if p_val < val && q_val < val {
                node = curr.borrow().left.clone();
            } else if p_val > val && q_val > val {
                node = curr.borrow().right.clone();
            } else {
                return node;
            }
        }

        None
    }
}

// @lc code=end

// Warning: this is a manual question, the generated test code may be incorrect.
fn main() -> Result<()> {
    let root: BinaryTree = deserialize(&read_line()?)?;
    let p: i32 = deserialize(&read_line()?)?;
    let q: i32 = deserialize(&read_line()?)?;
    let ans: BinaryTree = Solution::lowest_common_ancestor(root.into(), p, q).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
