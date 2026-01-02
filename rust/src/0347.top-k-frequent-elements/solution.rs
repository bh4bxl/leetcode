// Created by bh4bxl at 2025/12/29 18:07
// leetgo: 1.4.16
// https://leetcode.com/problems/top-k-frequent-elements/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut cnt_map = HashMap::new();

        for n in nums {
            cnt_map.entry(n).and_modify(|v| *v += 1).or_insert(1);
        }

        let mut sort_vec: Vec<_> = cnt_map.iter().collect();

        sort_vec.sort_by(|a, b| b.1.cmp(a.1));

        let mut res = vec![];

        for i in 0..k as usize {
            res.push(*sort_vec[i].0);
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let k: i32 = deserialize(&read_line()?)?;
    let ans: Vec<i32> = Solution::top_kfrequent(nums, k).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
