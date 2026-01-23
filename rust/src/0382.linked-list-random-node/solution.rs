// Created by bh4bxl at 2026/01/19 20:14
// leetgo: 1.4.16
// https://leetcode.com/problems/linked-list-random-node/

use anyhow::Result;
use leetgo_rs::*;

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
use rand::Rng;

struct Solution {
    head: Option<Box<ListNode>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(head: Option<Box<ListNode>>) -> Self {
        Self { head }
    }

    fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let (mut res, mut cnt) = (0, 0);
        let mut node = &self.head;

        while let Some(n) = node {
            cnt += 1;
            if rng.gen_range(0..cnt) == 0 {
                res = n.val;
            }
            node = &n.next;
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let ops: Vec<String> = deserialize(&read_line()?)?;
    let params = split_array(&read_line()?)?;
    let mut output = Vec::with_capacity(ops.len());
    output.push("null".to_string());

    let constructor_params = split_array(&params[0])?;
    let head: LinkedList = deserialize(&constructor_params[0])?;
    #[allow(unused_mut)]
    let mut obj = Solution::new(head.into());

    for i in 1..ops.len() {
        match ops[i].as_str() {
            "getRandom" => {
                let ans: i32 = obj.get_random().into();
                output.push(serialize(ans)?);
            }
            _ => panic!("unknown op"),
        }
    }

    println!("\noutput: {}", join_array(output));
    Ok(())
}
