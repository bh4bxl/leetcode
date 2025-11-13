// Created by bh4bxl at 2025/11/11 17:19
// leetgo: 1.4.15
// https://leetcode.com/problems/product-of-array-except-self/

use std::usize;

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; nums.len()];
        let mut prod = 1;
        let mut zeros = 0;
        let mut zero_pos = usize::MAX;

        for i in 0..nums.len() {
            if nums[i] == 0 {
                zeros += 1;
                zero_pos = i;
                if zeros == 2 {
                    return res;
                }
            } else {
                prod *= nums[i];
            }
        }

        if zeros == 1 {
            res[zero_pos] = prod;
            return res;
        }

        for (i, n) in nums.iter().enumerate() {
            res[i] = prod / n;
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: Vec<i32> = Solution::product_except_self(nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
