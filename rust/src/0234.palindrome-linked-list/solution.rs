// Created by bh4bxl at 2025/11/10 22:44
// leetgo: 1.4.15
// https://leetcode.com/problems/palindrome-linked-list/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut curr = &mut head.clone();
        let mut array = vec![];

        while let Some(node) = curr {
            array.push(node.val);
            curr = &mut node.next;
        }

        for i in 0..array.len() / 2 {
            if array[i] != array[array.len() - 1 - i] {
                return false;
            }
        }

        true
    }
}

// @lc code=end

fn main() -> Result<()> {
    let head: LinkedList = deserialize(&read_line()?)?;
    let ans: bool = Solution::is_palindrome(head.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
