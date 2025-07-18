// Created by bh4bxl at 2025/07/16 22:11
// leetgo: 1.4.15
// https://leetcode.com/problems/4sum/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nums_m = nums.clone();
        nums_m.sort();

        let mut res: Vec<Vec<i32>> = Vec::new();
        if nums.len() < 3 {
            return res;
        }

        for j in 0..nums.len() - 3 {
            if j != 0 && !(j > 0 && nums_m[j] != nums_m[j - 1]) {
                continue;
            }

            for i in j + 1..nums.len() - 2 {
                if i != j + 1 && nums_m[i] == nums_m[i - 1] {
                    continue;
                }

                let mut lo = i + 1;
                let mut hi = nums.len() - 1;
                let sum = target as i64 - nums_m[j] as i64 - nums_m[i] as i64;
                while lo < hi {
                    if nums_m[lo] as i64 + nums_m[hi] as i64 == sum {
                        res.push(
                            vec![nums_m[j], nums_m[i],
                                 nums_m[lo], nums_m[hi]]
                        );

                        while lo < hi && nums_m[lo] == nums_m[lo + 1] {
                            lo += 1;
                        }
                        while lo < hi && nums_m[hi] == nums_m[hi - 1] {
                            hi -= 1;
                        }
                        lo += 1;
                        hi -= 1;
                    } else if nums_m[lo] as i64 + (nums_m[hi] as i64) < sum {
                        lo += 1;
                    } else {
                        hi -= 1;
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
	let target: i32 = deserialize(&read_line()?)?;
	let ans: Vec<Vec<i32>> = Solution::four_sum(nums, target).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
