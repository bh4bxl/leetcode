// Created by bh4bxl at 2025/10/21 11:11
// leetgo: 1.4.15
// https://leetcode.com/problems/course-schedule/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let n = num_courses as usize;
        let mut graph = vec![vec![]; n];
        let mut indegree = vec![0; n];

        for p in prerequisites {
            let (a, b) = (p[0] as usize, p[1] as usize);
            graph[b].push(a);
            indegree[a] += 1;
        }

        let mut queue = std::collections::VecDeque::new();
        for i in 0..n {
            if indegree[i] == 0 {
                queue.push_back(i);
            }
        }

        let mut finished = 0;

        while let Some(course) = queue.pop_front() {
            finished += 1;
            for &next in &graph[course] {
                indegree[next] -= 1;
                if indegree[next] == 0 {
                    queue.push_back(next);
                }
            }
        }

        finished == n
    }
}

// @lc code=end

fn main() -> Result<()> {
    let num_courses: i32 = deserialize(&read_line()?)?;
    let prerequisites: Vec<Vec<i32>> = deserialize(&read_line()?)?;
    let ans: bool = Solution::can_finish(num_courses, prerequisites).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
