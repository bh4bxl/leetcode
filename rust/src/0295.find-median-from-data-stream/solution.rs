// Created by bh4bxl at 2025/12/01 14:17
// leetgo: 1.4.15
// https://leetcode.com/problems/find-median-from-data-stream/

use anyhow::Result;
use leetgo_rs::*;

// @lc code=begin
use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct MedianFinder {
    left: BinaryHeap<i32>,
    right: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    fn new() -> Self {
        Self {
            left: BinaryHeap::new(),
            right: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        if self.left.peek().map_or(true, |&x| num <= x) {
            self.left.push(num);
        } else {
            self.right.push(Reverse(num));
        }

        if self.left.len() > self.right.len() + 1 {
            let val = self.left.pop().unwrap();
            self.right.push(Reverse(val));
        } else if self.right.len() > self.left.len() {
            let Reverse(val) = self.right.pop().unwrap();
            self.left.push(val);
        }
    }

    fn find_median(&self) -> f64 {
        if self.left.len() > self.right.len() {
            *self.left.peek().unwrap() as f64
        } else {
            let a = *self.left.peek().unwrap() as f64;
            let b = self.right.peek().unwrap().0 as f64;
            (a + b) / 2.0
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let ops: Vec<String> = deserialize(&read_line()?)?;
    let params = split_array(&read_line()?)?;
    let mut output = Vec::with_capacity(ops.len());
    output.push("null".to_string());

    #[allow(unused_mut)]
    let mut obj = MedianFinder::new();

    for i in 1..ops.len() {
        match ops[i].as_str() {
            "addNum" => {
                let method_params = split_array(&params[i])?;
                let num: i32 = deserialize(&method_params[0])?;
                obj.add_num(num);
                output.push("null".to_string());
            }
            "findMedian" => {
                let ans: f64 = obj.find_median().into();
                output.push(serialize(ans)?);
            }
            _ => panic!("unknown op"),
        }
    }

    println!("\noutput: {}", join_array(output));
    Ok(())
}
