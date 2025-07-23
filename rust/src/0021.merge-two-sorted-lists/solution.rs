// Created by bh4bxl at 2025/07/17 21:46
// leetgo: 1.4.15
// https://leetcode.com/problems/merge-two-sorted-lists/

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
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut merged = Some(Box::new(ListNode { next: None, val: 0 }));
        let mut cur = &mut merged;
        let mut l1_m = list1.clone();
        let mut l2_m = list2.clone();
        let mut cur1 = &mut l1_m;
        let mut cur2 = &mut l2_m;

        while cur1.is_some() && cur2.is_some() {
            if cur1.as_ref()?.val < cur2.as_ref()?.val {
                cur.as_mut()?.next = cur1.clone();
                cur1 = &mut cur1.as_mut()?.next;
            } else {
                cur.as_mut()?.next = cur2.clone();
                cur2= &mut cur2.as_mut()?.next
            }
            cur = &mut cur.as_mut()?.next;
        }

        if cur1.is_some() {
            cur.as_mut()?.next = cur1.clone();
        }
        if cur2.is_some() {
            cur.as_mut()?.next = cur2.clone();
        }

        merged.unwrap().next
    }
}

// @lc code=end

fn main() -> Result<()> {
	let list1: LinkedList = deserialize(&read_line()?)?;
	let list2: LinkedList = deserialize(&read_line()?)?;
	let ans: LinkedList = Solution::merge_two_lists(list1.into(), list2.into()).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
