// Created by bh4bxl at 2025/07/18 19:30
// leetgo: 1.4.15
// https://leetcode.com/problems/merge-k-sorted-lists/

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
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut cur = &mut list1;

        while list2.is_some() {
            if cur.is_none() || list2.as_ref()?.val < cur.as_ref()?.val {
                std::mem::swap(cur, &mut list2);
            }
            cur = &mut cur.as_mut()?.next
        }
        list1
    }

    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.len() == 0 {
            return None;
        }
        if lists.len() == 1 {
            return lists[0].clone();
        }

        let mut head = lists[0].clone();
        for i in 1..lists.len() {
            let list = lists[i].clone();
            head = Self::merge_two_lists(head, list);
        }

        head
    }
}

// @lc code=end

fn main() -> Result<()> {
    let lists: Vec<LinkedList> = deserialize(&read_line()?)?;
    let ans: LinkedList = Solution::merge_k_lists(lists).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
