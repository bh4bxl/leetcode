// Created by bh4bxl at 2025/09/23 15:21
// leetgo: 1.4.15
// https://leetcode.com/problems/sort-list/

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
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }

        // Count the number of nodes in the list
        let mut len = 0;
        let mut curr = head.as_ref();
        while let Some(node) = curr {
            len += 1;
            curr = node.next.as_ref();
        }

        // Split the list into two halves
        let mid = len / 2;
        let mut head = head;
        let mut left_half_tail = head.as_mut().unwrap();
        for _ in 0..mid - 1 {
            left_half_tail = left_half_tail.next.as_mut().unwrap();
        }

        let right_half = left_half_tail.next.take();

        // Recursively sort both halves
        let sorted_left = Self::sort_list(head);
        let sorted_right = Self::sort_list(right_half);

        // Merge the sorted halves
        Self::merge(sorted_left, sorted_right)
    }

    fn merge(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: None });
        let mut tail = &mut dummy;

        let mut l1 = l1;
        let mut l2 = l2;

        while l1.is_some() && l2.is_some() {
            if l1.as_ref().unwrap().val < l2.as_ref().unwrap().val {
                let mut node = l1.unwrap();
                l1 = node.next.take();
                tail.next = Some(node);
            } else {
                let mut node = l2.unwrap();
                l2 = node.next.take();
                tail.next = Some(node);
            }
            tail = tail.next.as_mut().unwrap();
        }

        tail.next = if l1.is_some() { l1 } else { l2 };
        dummy.next
    }
}

// @lc code=end

fn main() -> Result<()> {
    let head: LinkedList = deserialize(&read_line()?)?;
    let ans: LinkedList = Solution::sort_list(head.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
