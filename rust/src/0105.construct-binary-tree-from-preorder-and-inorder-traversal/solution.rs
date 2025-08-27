// Created by bh4bxl at 2025/08/26 12:14
// leetgo: 1.4.15
// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/

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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut node_map = std::collections::HashMap::new();

        for (i, n) in inorder.iter().enumerate() {
            node_map.insert(*n, i);
        }

        Self::create_tree(
            &preorder,
            0,
            preorder.len() - 1,
            &inorder,
            0,
            inorder.len() - 1,
            &node_map,
        )
    }

    fn create_tree(
        preorder: &Vec<i32>,
        p_start: usize,
        p_end: usize,
        inorder: &Vec<i32>,
        i_start: usize,
        i_end: usize,
        node_map: &std::collections::HashMap<i32, usize>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if p_start > p_end || i_start > i_end {
            return None;
        }

        let root_val = preorder[p_start];
        let root_idx = node_map.get(&root_val).unwrap();
        let len = root_idx - i_start;
        let node = Rc::new(RefCell::new(TreeNode {
            val: root_val,
            left: Self::create_tree(
                preorder,
                p_start + 1,
                p_start + len,
                inorder,
                i_start,
                if *root_idx == 0 { 0 } else { root_idx - 1 },
                node_map,
            ),
            right: Self::create_tree(
                preorder,
                p_start + len + 1,
                p_end,
                inorder,
                root_idx + 1,
                i_end,
                node_map,
            ),
        }));

        Some(node)
    }
}

// @lc code=end

fn main() -> Result<()> {
    let preorder: Vec<i32> = deserialize(&read_line()?)?;
    let inorder: Vec<i32> = deserialize(&read_line()?)?;
    let ans: BinaryTree = Solution::build_tree(preorder, inorder).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
