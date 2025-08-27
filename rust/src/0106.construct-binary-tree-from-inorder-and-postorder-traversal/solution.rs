// Created by bh4bxl at 2025/08/26 14:13
// leetgo: 1.4.15
// https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/

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
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut map = std::collections::HashMap::new();

        for (i, n) in inorder.iter().enumerate() {
            map.insert(*n, i as i32);
        }

        Self::create_tree(
            &inorder,
            0,
            inorder.len() as i32 - 1,
            &postorder,
            0,
            postorder.len() as i32 - 1,
            &map,
        )
    }

    fn create_tree(
        inorder: &Vec<i32>,
        istart: i32,
        iend: i32,
        postorder: &Vec<i32>,
        pstart: i32,
        pend: i32,
        map: &std::collections::HashMap<i32, i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if istart > iend || pstart > pend {
            return None;
        }

        let root_val = postorder[pend as usize];
        let root_idx = map.get(&root_val).unwrap();
        let len = iend - root_idx;
        Some(Rc::new(RefCell::new(TreeNode {
            val: root_val,
            left: Self::create_tree(
                inorder,
                istart,
                root_idx - 1,
                postorder,
                pstart,
                pend - len - 1,
                map,
            ),
            right: Self::create_tree(
                inorder,
                root_idx + 1,
                iend,
                postorder,
                pend - len,
                pend - 1,
                map,
            ),
        })))
    }
}

// @lc code=end

fn main() -> Result<()> {
    let inorder: Vec<i32> = deserialize(&read_line()?)?;
    let postorder: Vec<i32> = deserialize(&read_line()?)?;
    let ans: BinaryTree = Solution::build_tree(inorder, postorder).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
