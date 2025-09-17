// Created by bh4bxl at 2025/09/15 14:43
// leetgo: 1.4.15
// https://leetcode.com/problems/gas-station/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let len = gas.len();
        let mut sum = 0;
        let mut idx = 0;
        let mut total = 0;

        for i in 0..len {
            sum += gas[i] - cost[i];
            if sum < 0 {
                total += sum;
                sum = 0;
                idx = i + 1;
            }
        }

        total += sum;

        if total < 0 { -1 } else { idx as i32 }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let gas: Vec<i32> = deserialize(&read_line()?)?;
    let cost: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::can_complete_circuit(gas, cost).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
