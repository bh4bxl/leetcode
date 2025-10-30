// Created by bh4bxl at 2025/10/27 20:07
// leetgo: 1.4.15
// https://leetcode.com/problems/kth-largest-element-in-an-array/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = std::collections::BinaryHeap::new();

        for &num in &nums {
            heap.push(std::cmp::Reverse(num));
            if heap.len() > k as usize {
                heap.pop();
            }
        }

        heap.peek().unwrap().0
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let k: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::find_kth_largest(nums, k).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
