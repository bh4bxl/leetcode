// Created by bh4bxl at 2025/08/30 15:33
// leetgo: 1.4.15
// https://leetcode.com/problems/flatten-binary-tree-to-linked-list/

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
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if root.is_none() {
            return;
        }
        let mut queue = vec![];
        Self::inorder(&root, &mut queue);

        let mut curr = Rc::clone(root.as_ref().unwrap());
        curr.borrow_mut().left = None;

        for val in &queue[1..] {
            let new_node = Rc::new(RefCell::new(TreeNode {
                val: *val,
                left: None,
                right: None,
            }));
            curr.borrow_mut().right = Some(Rc::clone(&new_node));
            curr = new_node;
        }
    }

    fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, queue: &mut Vec<i32>) {
        if let Some(n) = node {
            let np = n.borrow();
            let val = np.val;
            queue.push(val);
            Self::inorder(&np.left, queue);
            Self::inorder(&np.right, queue);
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let root: BinaryTree = deserialize(&read_line()?)?;
    Solution::flatten(root.into());
    let ans: BinaryTree = root.into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
