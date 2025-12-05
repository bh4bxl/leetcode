// Created by bh4bxl at 2025/12/01 14:39
// leetgo: 1.4.15
// https://leetcode.com/problems/serialize-and-deserialize-binary-tree/

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
struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, out: &mut String) {
            if let Some(n) = node {
                let np = n.borrow();
                out.push_str(&np.val.to_string());
                out.push(',');
                dfs(&np.left, out);
                dfs(&np.right, out);
            } else {
                out.push_str("null,");
            }
        }

        let mut res = String::new();
        dfs(&root, &mut res);

        res
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        fn build(tokens: &mut std::str::Split<'_, char>) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(val) = tokens.next() {
                if val == "null" {
                    return None;
                }

                let num = val.parse::<i32>().unwrap();
                let node = Rc::new(RefCell::new(TreeNode::new(num)));
                node.borrow_mut().left = build(tokens);
                node.borrow_mut().right = build(tokens);
                Some(node)
            } else {
                None
            }
        }

        let mut iter = data.split(',');
        build(&mut iter)
    }
}

// @lc code=end

// Warning: this is a manual question, the generated test code may be incorrect.
fn main() -> Result<()> {
    let root: BinaryTree = deserialize(&read_line()?)?;
    let ans: String = Solution::codec(root.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
