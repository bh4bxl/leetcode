// Created by bh4bxl at 2025/11/11 17:34
// leetgo: 1.4.15
// https://leetcode.com/problems/sliding-window-maximum/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::VecDeque;
        let mut deque: VecDeque<usize> = VecDeque::new();
        let mut res = vec![];
        let k = k as usize;

        for i in 0..nums.len() {
            if let Some(&front) = deque.front() {
                if front + k <= i {
                    deque.pop_front();
                }
            }

            while let Some(&back) = deque.back() {
                if nums[back] < nums[i] {
                    deque.pop_back();
                } else {
                    break;
                }
            }

            deque.push_back(i);

            if i >= k - 1 {
                res.push(nums[*deque.front().unwrap()]);
            }
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let k: i32 = deserialize(&read_line()?)?;
    let ans: Vec<i32> = Solution::max_sliding_window(nums, k).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
