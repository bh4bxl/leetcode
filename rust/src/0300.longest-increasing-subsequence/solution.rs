// Created by bh4bxl at 2025/12/01 15:14
// leetgo: 1.4.15
// https://leetcode.com/problems/longest-increasing-subsequence/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut tails: Vec<i32> = vec![];

        for n in nums {
            let (mut left, mut right) = (0usize, tails.len());

            while left < right {
                let mid = left + (right - left) / 2;
                if tails[mid] < n {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }

            if left == tails.len() {
                tails.push(n);
            } else {
                tails[left] = n;
            }
        }

        tails.len() as i32
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::length_of_lis(nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
