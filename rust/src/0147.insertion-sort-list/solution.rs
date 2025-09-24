// Created by bh4bxl at 2025/09/23 10:47
// leetgo: 1.4.15
// https://leetcode.com/problems/insertion-sort-list/

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
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: None });
        let mut curr = head;

        while let Some(mut node) = curr {
            curr = node.next.take();

            let mut prev = &mut dummy;
            while let Some(ref next) = prev.next {
                if next.val >= node.val {
                    break;
                }
                prev = prev.next.as_mut().unwrap();
            }

            node.next = prev.next.take();
            prev.next = Some(node);
        }

        dummy.next
    }
}

// @lc code=end

fn main() -> Result<()> {
    let head: LinkedList = deserialize(&read_line()?)?;
    let ans: LinkedList = Solution::insertion_sort_list(head.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
