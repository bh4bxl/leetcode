// Created by bh4bxl at 2025/10/14 12:32
// leetgo: 1.4.15
// https://leetcode.com/problems/binary-search-tree-iterator/

use anyhow::Result;
use leetgo_rs::*;

// @lc code=begin
use std::{cell::RefCell, rc::Rc};

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
struct BSTIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut it = BSTIterator { stack: vec![] };
        it.push_left(root);
        it
    }

    fn next(&mut self) -> i32 {
        let node_rc = self.stack.pop().expect("next called but no next element");
        let val = node_rc.borrow().val;
        let right = node_rc.borrow().right.clone();
        self.push_left(right);
        val
    }

    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }

    fn push_left(&mut self, mut node_opt: Option<Rc<RefCell<TreeNode>>>) {
        while let Some(node_rc) = node_opt {
            self.stack.push(node_rc.clone());
            node_opt = node_rc.borrow().left.clone();
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let ops: Vec<String> = deserialize(&read_line()?)?;
    let params = split_array(&read_line()?)?;
    let mut output = Vec::with_capacity(ops.len());
    output.push("null".to_string());

    let constructor_params = split_array(&params[0])?;
    let root: BinaryTree = deserialize(&constructor_params[0])?;
    #[allow(unused_mut)]
    let mut obj = BSTIterator::new(root.into());

    for i in 1..ops.len() {
        match ops[i].as_str() {
            "next" => {
                let ans: i32 = obj.next().into();
                output.push(serialize(ans)?);
            }
            "hasNext" => {
                let ans: bool = obj.has_next().into();
                output.push(serialize(ans)?);
            }
            _ => panic!("unknown op"),
        }
    }

    println!("\noutput: {}", join_array(output));
    Ok(())
}
