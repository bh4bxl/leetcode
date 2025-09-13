// Created by bh4bxl at 2025/08/30 12:25
// leetgo: 1.4.15
// https://leetcode.com/problems/path-sum-ii/

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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }

        let mut res: Vec<Vec<i32>> = vec![];
        let mut path = vec![];

        Self::inorder(&root, 0, target_sum, &mut path, &mut res);

        res
    }

    fn inorder(
        node: &Option<Rc<RefCell<TreeNode>>>,
        sum: i32,
        target_sum: i32,
        path: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
    ) {
        if let Some(n) = node {
            let np = n.borrow();
            let val = np.val;
            path.push(val);
            if np.left.is_none() && np.right.is_none() {
                if sum + val == target_sum {
                    res.push(path.clone());
                }
            }
            Self::inorder(&np.left, sum + val, target_sum, path, res);
            Self::inorder(&np.right, sum + val, target_sum, path, res);
            path.pop();
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let root: BinaryTree = deserialize(&read_line()?)?;
    let target_sum: i32 = deserialize(&read_line()?)?;
    let ans: Vec<Vec<i32>> = Solution::path_sum(root.into(), target_sum).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
