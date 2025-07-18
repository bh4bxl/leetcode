// Created by bh4bxl at 2025/07/10 15:54
// leetgo: 1.4.14
// https://leetcode.com/problems/add-two-numbers/

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
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut list1 = l1;
        let mut list2 = l2;
        let mut res: Option<Box<ListNode>> = None;
        let mut curr = &mut res; 
        let mut carry = 0;

        while list1.is_some() || list2.is_some() || carry != 0 {
            let n1 = list1.as_ref().map_or(0, |x| x.val);
            let n2 = list2.as_ref().map_or(0, |x| x.val);
            let sum = n1 + n2 + carry;
            let sum_noode = curr.insert(Box::new(ListNode::new(sum % 10)));
            curr = &mut sum_noode.next;
            carry = sum / 10;
            list1 = list1.and_then(|node| node.next);
            list2 = list2.and_then(|node| node.next);
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
	let l1: LinkedList = deserialize(&read_line()?)?;
	let l2: LinkedList = deserialize(&read_line()?)?;
	let ans: LinkedList = Solution::add_two_numbers(l1.into(), l2.into()).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
