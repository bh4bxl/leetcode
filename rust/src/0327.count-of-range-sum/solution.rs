// Created by bh4bxl at 2025/12/15 19:53
// leetgo: 1.4.15
// https://leetcode.com/problems/count-of-range-sum/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let mut prefix = vec![0i64];
        for &x in &nums {
            prefix.push(prefix.last().unwrap() + x as i64);
        }

        let (lower, upper) = (lower as i64, upper as i64);

        fn sort_count(
            sums: &mut Vec<i64>,
            left: usize,
            right: usize,
            lower: i64,
            upper: i64,
        ) -> i32 {
            if right - left <= 1 {
                return 0;
            }

            let mid = (left + right) / 2;
            let mut count = sort_count(sums, left, mid, lower, upper)
                + sort_count(sums, mid, right, lower, upper);

            let (mut i, mut j) = (mid, mid);

            for l in left..mid {
                while i < right && sums[i] - sums[l] < lower {
                    i += 1;
                }
                while j < right && sums[j] - sums[l] <= upper {
                    j += 1;
                }
                count += (j - i) as i32;
            }

            let mut merged = Vec::with_capacity(right - left);
            let (mut p, mut q) = (left, mid);

            while p < mid && q < right {
                if sums[p] <= sums[q] {
                    merged.push(sums[p]);
                    p += 1;
                } else {
                    merged.push(sums[q]);
                    q += 1;
                }
            }

            merged.extend_from_slice(&sums[p..mid]);
            merged.extend_from_slice(&sums[q..right]);

            sums[left..right].copy_from_slice(&merged);

            count
        }

        let len = prefix.len();
        sort_count(&mut prefix, 0, len, lower, upper)
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let lower: i32 = deserialize(&read_line()?)?;
    let upper: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::count_range_sum(nums, lower, upper).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
