// Created by bh4bxl at 2025/08/22 10:35
// leetgo: 1.4.15
// https://leetcode.com/problems/same-tree/

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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        Self::check(&p, &q)
    }

    fn check(
        node_p: &Option<Rc<RefCell<TreeNode>>>,
        node_q: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if node_p.is_none() && node_q.is_none() {
            return true;
        } else if node_p.is_none() || node_q.is_none() {
            return false;
        }

        if let (Some(p), Some(q)) = (node_p, node_q) {
            let (np, nq) = (p.borrow(), q.borrow());
            let (vp, vq) = (np.val, nq.val);

            if !Self::check(&np.left, &nq.left) {
                return false;
            }

            if vp != vq {
                return false;
            }

            if !Self::check(&np.right, &nq.right) {
                return false;
            }
        }

        true
    }
}

// @lc code=end

fn main() -> Result<()> {
    let p: BinaryTree = deserialize(&read_line()?)?;
    let q: BinaryTree = deserialize(&read_line()?)?;
    let ans: bool = Solution::is_same_tree(p.into(), q.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
