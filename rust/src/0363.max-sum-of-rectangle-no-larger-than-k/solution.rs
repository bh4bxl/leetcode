// Created by bh4bxl at 2026/01/05 20:31
// leetgo: 1.4.16
// https://leetcode.com/problems/max-sum-of-rectangle-no-larger-than-k/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

use std::collections::BTreeSet;
impl Solution {
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut res = i32::MIN;

        // Iterate by smaller dimension for efficiency
        if m <= n {
            // Fix top and bottom rows
            for top in 0..m {
                let mut col_sum = vec![0; n];
                for bottom in top..m {
                    for c in 0..n {
                        col_sum[c] += matrix[bottom][c];
                    }
                    res = res.max(Self::max_subarray_no_more_than_k(&col_sum, k));
                }
            }
        } else {
            // Fix left and right colums
            for left in 0..n {
                let mut row_sum = vec![0; m];
                for right in left..n {
                    for r in 0..m {
                        row_sum[r] += matrix[r][right];
                    }
                    res = res.max(Self::max_subarray_no_more_than_k(&row_sum, k));
                }
            }
        }

        res
    }

    fn max_subarray_no_more_than_k(nums: &Vec<i32>, k: i32) -> i32 {
        let mut set = BTreeSet::new();
        set.insert(0);

        let mut prefix = 0;
        let mut res = i32::MIN;

        for &x in nums {
            prefix += x;
            if let Some(&p) = set.range(prefix - k..).next() {
                res = res.max(prefix - p);
            }
            set.insert(prefix);
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let matrix: Vec<Vec<i32>> = deserialize(&read_line()?)?;
    let k: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::max_sum_submatrix(matrix, k).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
