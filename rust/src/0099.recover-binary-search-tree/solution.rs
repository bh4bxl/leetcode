// Created by bh4bxl at 2025/08/21 21:15
// leetgo: 1.4.15
// https://leetcode.com/problems/recover-binary-search-tree/

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
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut prev = None;
        let mut first = None;
        let mut second = None;

        Self::inorder(root, &mut prev, &mut first, &mut second);

        if let (Some(f), Some(s)) = (first, second) {
            std::mem::swap(&mut f.borrow_mut().val, &mut s.borrow_mut().val);
        }
    }

    fn inorder<'a>(
        node: &Option<Rc<RefCell<TreeNode>>>,
        prev: &mut Option<Rc<RefCell<TreeNode>>>,
        first: &mut Option<Rc<RefCell<TreeNode>>>,
        second: &mut Option<Rc<RefCell<TreeNode>>>,
    ) {
        if let Some(rc) = node {
            let n = rc.borrow();
            Self::inorder(&n.left, prev, first, second);

            if let Some(p) = prev {
                if p.borrow().val > n.val {
                    if first.is_none() {
                        *first = Some(Rc::clone(p));
                    }
                    *second = Some(Rc::clone(rc));
                }
            }

            *prev = Some(Rc::clone(rc));

            Self::inorder(&n.right, prev, first, second);
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let root: BinaryTree = deserialize(&read_line()?)?;
    Solution::recover_tree(root.into());
    let ans: BinaryTree = root.into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
