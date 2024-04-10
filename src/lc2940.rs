pub struct Solution;

impl Solution {
    pub fn leftmost_building_queries(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut desc_stack = Vec::new();
        let mut next_height_idx = vec![None; heights.len()];
        for (i, &h) in heights.iter().enumerate() {
            while let Some(&last) = desc_stack.last() {
                if h <= heights[last] {
                    break;
                }
                next_height_idx[last] = Some(i);
                desc_stack.pop();
            }
            desc_stack.push(i);
        }
        let result = queries
            .into_iter()
            .map(|e| {
                let (alice, bob) = (e[0] as usize, e[1] as usize);
                if alice == bob {
                    return alice as i32;
                }
                let mut idx = alice.max(bob);
                let height = heights[alice].max(heights[bob]);
                if idx == bob && heights[bob] > heights[alice]
                    || idx == alice && heights[alice] > heights[bob]
                {
                    return idx as i32;
                }
                while let Some(next) = next_height_idx[idx] {
                    if heights[next] > height {
                        return next as i32;
                    }
                    idx = next;
                }
                -1
            })
            .collect::<Vec<_>>();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_1() {
        assert_eq!(
            Solution::leftmost_building_queries(
                vec![6, 4, 8, 5, 2, 7],
                vec![vec![0, 1], vec![0, 3], vec![2, 4], vec![3, 4], vec![2, 2]]
            ),
            vec![2, 5, -1, 5, 2]
        )
    }

    #[test]
    fn testcase_2() {
        assert_eq!(
            Solution::leftmost_building_queries(
                vec![5, 3, 8, 2, 6, 1, 4, 6],
                vec![vec![0, 7], vec![3, 5], vec![5, 2], vec![3, 0], vec![1, 6]]
            ),
            vec![7, 6, -1, 4, 6]
        )
    }

    #[test]
    fn testcase_3() {
        assert_eq!(
            Solution::leftmost_building_queries(
                vec![1, 2, 1, 2, 1, 2],
                vec![
                    vec![0, 0],
                    vec![0, 1],
                    vec![0, 2],
                    vec![0, 3],
                    vec![0, 4],
                    vec![0, 5],
                    vec![1, 0],
                    vec![1, 1],
                    vec![1, 2],
                    vec![1, 3],
                    vec![1, 4],
                    vec![1, 5],
                    vec![2, 0],
                    vec![2, 1],
                    vec![2, 2],
                    vec![2, 3],
                    vec![2, 4],
                    vec![2, 5],
                    vec![3, 0],
                    vec![3, 1],
                    vec![3, 2],
                    vec![3, 3],
                    vec![3, 4],
                    vec![3, 5],
                    vec![4, 0],
                    vec![4, 1],
                    vec![4, 2],
                    vec![4, 3],
                    vec![4, 4],
                    vec![4, 5],
                    vec![5, 0],
                    vec![5, 1],
                    vec![5, 2],
                    vec![5, 3],
                    vec![5, 4],
                    vec![5, 5]
                ]
            ),
            vec![
                0, 1, 3, 3, 5, 5, 1, 1, -1, -1, -1, -1, 3, -1, 2, 3, 5, 5, 3, -1, 3, 3, -1, -1, 5,
                -1, 5, -1, 4, 5, 5, -1, 5, -1, 5, 5
            ]
        )
    }
}
