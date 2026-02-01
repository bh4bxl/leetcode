// Created by bh4bxl at 2026/01/27 16:42
// leetgo: 1.4.16
// https://leetcode.com/problems/perfect-rectangle/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::collections::HashSet;

impl Solution {
    pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;
        let mut area = 0i64;
        let mut corners = HashSet::new();

        for r in rectangles {
            let (x1, y1, x2, y2) = (r[0], r[1], r[2], r[3]);

            min_x = min_x.min(x1);
            min_y = min_y.min(y1);
            max_x = max_x.max(x2);
            max_y = max_y.max(y2);

            area += (x2 - x1) as i64 * (y2 - y1) as i64;

            let pts = [(x1, y1), (x1, y2), (x2, y1), (x2, y2)];

            for p in pts {
                if !corners.insert(p) {
                    corners.remove(&p);
                }
            }
        }

        let expected_area = (max_x - min_x) as i64 * (max_y - min_y) as i64;

        if area != expected_area {
            return false;
        }

        if corners.len() != 4 {
            return false;
        }

        corners.contains(&(min_x, min_y))
            && corners.contains(&(min_x, max_y))
            && corners.contains(&(max_x, min_y))
            && corners.contains(&(max_x, max_y))
    }
}

// @lc code=end

fn main() -> Result<()> {
    let rectangles: Vec<Vec<i32>> = deserialize(&read_line()?)?;
    let ans: bool = Solution::is_rectangle_cover(rectangles).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
