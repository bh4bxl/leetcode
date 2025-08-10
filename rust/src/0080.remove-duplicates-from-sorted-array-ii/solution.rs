// Created by bh4bxl at 2025/08/10 13:33
// leetgo: 1.4.15
// https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/

use std::i32;

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut dups = vec![];
        let mut num = i32::MIN;

        for i in nums.iter() {
            if *i != num {
                dups.push(1);
                num = *i;
            } else {
                let last = dups.len() - 1;
                dups[last] += 1;
            }
        }

        let mut offset = 0;
        let mut idx = 0;

        for n in dups {
            nums[idx] = nums[idx + offset];
            idx += 1;
            if n > 1 {
                nums[idx] = nums[idx + offset];
                idx += 1;
                if n > 2 {
                    offset += n - 2;
                }
            }
        }

        (nums.len() - offset) as i32
    }
}

// @lc code=end

fn main() -> Result<()> {
    let mut nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::remove_duplicates(&mut nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
