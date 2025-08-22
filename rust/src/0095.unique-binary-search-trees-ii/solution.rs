// Created by bh4bxl at 2025/08/20 12:22
// leetgo: 1.4.15
// https://leetcode.com/problems/unique-binary-search-trees-ii/

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
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        Self::build(1, n)
    }

    fn build(start: i32, end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if start > end {
            return vec![None];
        }

        let mut res = vec![];

        for root_val in start..=end {
            let left_trees = Self::build(start, root_val - 1);
            let right_trees = Self::build(root_val + 1, end);

            for left in &left_trees {
                for right in &right_trees {
                    let node = Rc::new(RefCell::new(TreeNode {
                        val: root_val,
                        left: left.clone(),
                        right: right.clone(),
                    }));
                    res.push(Some(node));
                }
            }
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let n: i32 = deserialize(&read_line()?)?;
    let ans: Vec<BinaryTree> = Solution::generate_trees(n).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
