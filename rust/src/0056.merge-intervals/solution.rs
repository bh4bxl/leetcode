// Created by bh4bxl at 2025/07/31 10:38
// leetgo: 1.4.15
// https://leetcode.com/problems/merge-intervals/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals_m = intervals.clone();
        let mut res: Vec<Vec<i32>> = vec![];

        intervals_m.sort_by(|a, b| a[0].partial_cmp(&b[0]).unwrap());

        res.push(intervals_m[0].clone());

        for i in 1..intervals.len() {
            if intervals_m[i][0] <= res.last().unwrap()[1] {
                if intervals_m[i][1] > res.last().unwrap()[1] {
                    let e = res.pop().unwrap()[0];
                    res.push(vec![e, intervals_m[i][1]]);
                }
            } else {
                res.push(intervals_m[i].clone());
            }
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let intervals: Vec<Vec<i32>> = deserialize(&read_line()?)?;
    let ans: Vec<Vec<i32>> = Solution::merge(intervals).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
