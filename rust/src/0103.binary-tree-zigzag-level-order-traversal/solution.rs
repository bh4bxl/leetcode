// Created by bh4bxl at 2025/08/25 20:55
// leetgo: 1.4.15
// https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/

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
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];

        Self::pick_values(&root, 0, &mut res);

        for (i, l) in res.iter_mut().enumerate() {
            if i % 2 != 0 {
                l.reverse();
            }
        }

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
    let ans: Vec<Vec<i32>> = Solution::zigzag_level_order(root.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
