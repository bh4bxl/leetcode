// Created by bh4bxl at 2025/08/25 12:08
// leetgo: 1.4.15
// https://leetcode.com/problems/symmetric-tree/

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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }

        let n = root.as_ref().unwrap().borrow();

        let (left, right) = (n.left.clone(), n.right.clone());

        if left.is_none() && right.is_none() {
            return true;
        }

        if left.is_none() || right.is_none() {
            return false;
        }

        Self::check(&left, &right)
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

            if !Self::check(&np.left, &nq.right) {
                return false;
            }

            if vp != vq {
                return false;
            }

            if !Self::check(&np.right, &nq.left) {
                return false;
            }
        }

        true
    }
}

// @lc code=end

fn main() -> Result<()> {
    let root: BinaryTree = deserialize(&read_line()?)?;
    let ans: bool = Solution::is_symmetric(root.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
