// Created by bh4bxl at 2025/10/20 12:31
// leetgo: 1.4.15
// https://leetcode.com/problems/binary-tree-right-side-view/

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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        if root.is_none() {
            return res;
        }
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.unwrap());

        while !queue.is_empty() {
            let level_size = queue.len();
            for i in 0..level_size {
                if let Some(n) = queue.pop_front() {
                    let np = n.borrow();

                    if i == level_size - 1 {
                        res.push(np.val);
                    }

                    if let Some(left) = np.left.clone() {
                        queue.push_back(left);
                    }

                    if let Some(right) = np.right.clone() {
                        queue.push_back(right);
                    }
                }
            }
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let root: BinaryTree = deserialize(&read_line()?)?;
    let ans: Vec<i32> = Solution::right_side_view(root.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
