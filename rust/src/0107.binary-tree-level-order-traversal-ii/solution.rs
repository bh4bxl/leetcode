// Created by bh4bxl at 2025/08/26 14:44
// leetgo: 1.4.15
// https://leetcode.com/problems/binary-tree-level-order-traversal-ii/

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
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }

        let mut res = vec![];

        Self::pick_values(&root, 0, &mut res);

        res.reverse();

        res
    }

    fn pick_values(node: &Option<Rc<RefCell<TreeNode>>>, level: usize, res: &mut Vec<Vec<i32>>) {
        if node.is_none() {
            return;
        }

        if let Some(n) = node {
            let np = n.borrow();
            let v = np.val;
            if level >= res.len() {
                res.push(vec![v]);
            } else {
                res[level].push(v);
            }

            Self::pick_values(&np.left, level + 1, res);
            Self::pick_values(&np.right, level + 1, res);
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let root: BinaryTree = deserialize(&read_line()?)?;
    let ans: Vec<Vec<i32>> = Solution::level_order_bottom(root.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
