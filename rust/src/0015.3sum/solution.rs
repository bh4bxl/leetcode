// Created by bh4bxl at 2025/07/15 12:36
// leetgo: 1.4.15
// https://leetcode.com/problems/3sum/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums_m = nums.clone();
        nums_m.sort();

        let mut res: Vec<Vec<i32>> = Vec::new();

        for i in 0..nums_m.len() - 2 {
            if i == 0 || (i > 0 && nums_m[i] != nums_m[i - 1]) {
                let mut lo = i + 1;
                let mut hi = nums_m.len() - 1;
                let sum = 0 - nums_m[i];

                while lo < hi {
                    if nums_m[lo] + nums_m[hi] == sum {
                        res.push(vec![nums_m[i], nums_m[lo], nums_m[hi]]);

                        while lo < hi && nums_m[lo] == nums_m[lo + 1] {
                            lo += 1;
                        }
                        while lo < hi && nums_m[hi] == nums_m[hi - 1] {
                        hi -= 1;
                        }
                        lo += 1;
                        hi -= 1;
                    } else {
                        if nums_m[lo] + nums_m[hi] < sum {
                            lo += 1;
                        } else {
                            hi -= 1;
                        }
                    }
                }
            }
        }
        res
    }
}

// @lc code=end

fn main() -> Result<()> {
	let nums: Vec<i32> = deserialize(&read_line()?)?;
	let ans: Vec<Vec<i32>> = Solution::three_sum(nums).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
