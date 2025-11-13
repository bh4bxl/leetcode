// Created by bh4bxl at 2025/11/11 17:07
// leetgo: 1.4.15
// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/

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
        fn inorder(
            node: Option<Rc<RefCell<TreeNode>>>,
            p_val: i32,
            q_val: i32,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(n) = node.as_ref() {
                let np = n.borrow();
                if np.val == p_val || np.val == q_val {
                    return Some(n.clone());
                }

                let left = inorder(np.left.clone(), p_val, q_val);
                let right = inorder(np.right.clone(), p_val, q_val);

                match (left, right) {
                    (Some(_), Some(_)) => Some(n.clone()),
                    (Some(l), None) => Some(l),
                    (None, Some(r)) => Some(r),
                    _ => None,
                }
            } else {
                None
            }
        }
        inorder(
            root,
            p.as_ref().unwrap().borrow().val,
            q.as_ref().unwrap().borrow().val,
        )
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
