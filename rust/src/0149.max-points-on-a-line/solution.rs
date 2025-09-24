// Created by bh4bxl at 2025/09/24 14:36
// leetgo: 1.4.15
// https://leetcode.com/problems/max-points-on-a-line/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        if points.len() <= 2 {
            return points.len() as i32;
        }

        let mut result = 0;

        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 { a.abs() } else { gcd(b, a % b) }
        }

        for i in 0..points.len() {
            let mut map: std::collections::HashMap<(i32, i32), i32> =
                std::collections::HashMap::new();
            let mut overlap = 0;
            let mut local_max = 0;

            for j in (i + 1)..points.len() {
                let dx = points[j][0] - points[i][0];
                let dy = points[j][1] - points[i][1];

                if dx == 0 && dy == 0 {
                    overlap += 1;
                    continue;
                }

                let g = gcd(dx, dy);
                let mut dx = dx / g;
                let mut dy = dy / g;

                if dx < 0 {
                    dx = -dx;
                    dy = -dy;
                } else if dx == 0 {
                    dy = 1;
                } else if dy == 0 {
                    dx = 1;
                }

                let count = map.entry((dx, dy)).or_insert(0);
                *count += 1;
                local_max = local_max.max(*count);
            }

            result = result.max(local_max + overlap + 1);
        }

        result
    }
}

// @lc code=end

fn main() -> Result<()> {
    let points: Vec<Vec<i32>> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::max_points(points).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
