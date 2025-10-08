// Created by bh4bxl at 2025/10/07 11:02
// leetgo: 1.4.15
// https://leetcode.com/problems/maximum-gap/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 1 {
            return 0;
        }
        let (mut max, mut min) = (i32::MIN, i32::MAX);
        for &num in &nums {
            max = max.max(num);
            min = min.min(num);
        }
        if max == min {
            return 0;
        }

        let range = max as i64 - min as i64;
        let denom = n as i64 - 1;
        let bucket_size = ((range + denom - 1) / denom) as usize;

        let buckets = n - 1;
        let mut bucket_min = vec![i32::MAX; buckets];
        let mut bucket_max = vec![i32::MIN; buckets];
        let mut used = vec![false; buckets];

        for &num in &nums {
            if num == min || num == max {
                continue;
            }
            let idx = ((num as i64 - min as i64) / bucket_size as i64) as usize;

            if !used[idx] {
                bucket_min[idx] = num;
                bucket_max[idx] = num;
                used[idx] = true;
            } else {
                bucket_min[idx] = bucket_min[idx].min(num);
                bucket_max[idx] = bucket_max[idx].max(num);
            }
        }

        let mut max_gap = 0_i32;
        let mut prev = min;

        for i in 0..buckets {
            if !used[i] {
                continue;
            }
            let gap = bucket_min[i] - prev;
            max_gap = max_gap.max(gap);
            prev = bucket_max[i];
        }

        max_gap.max(max - prev)
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::maximum_gap(nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
