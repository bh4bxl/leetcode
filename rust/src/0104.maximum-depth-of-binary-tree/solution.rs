// Created by bh4bxl at 2025/08/26 12:02
// leetgo: 1.4.15
// https://leetcode.com/problems/maximum-depth-of-binary-tree/

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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut m_depth = 0;

        Self::inorder(&root, 0, &mut m_depth);

        m_depth as i32
    }

    fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, depth: usize, max_depth: &mut usize) {
        if node.is_none() {
            if depth > *max_depth {
                *max_depth = depth;
            }
        }

        if let Some(n) = node {
            let np = n.borrow();
            Self::inorder(&np.left, depth + 1, max_depth);
            Self::inorder(&np.right, depth + 1, max_depth);
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let root: BinaryTree = deserialize(&read_line()?)?;
    let ans: i32 = Solution::max_depth(root.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
