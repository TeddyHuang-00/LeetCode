pub struct Solution;

impl Solution {
    pub fn ways_to_partition(nums: Vec<i32>, k: i32) -> i32 {
        let mut dif = nums.iter().map(|&x| x as i64).sum::<i64>();
        let mut dif_map = std::collections::HashMap::new();
        let gain = nums.iter().map(|&x| k - x).collect::<Vec<_>>();
        (0..nums.len() - 1).for_each(|i| {
            dif -= 2 * nums[i] as i64;
            dif_map.entry(dif).or_insert(Vec::new()).push(i + 1);
        });
        let mut success = dif_map.get(&0).map_or(0, |v| v.len());
        (0..nums.len()).for_each(|i| {
            let on_left = dif_map.entry(gain[i] as i64).or_default().clone();
            let on_right = dif_map.entry(-gain[i] as i64).or_default().clone();
            success = success.max(
                on_left.len()
                    - on_left
                        .iter()
                        .enumerate()
                        .position(|(_, &x)| x > i)
                        .unwrap_or(on_left.len())
                    + on_right
                        .iter()
                        .enumerate()
                        .position(|(_, &x)| x > i)
                        .unwrap_or(on_right.len()),
            );
        });
        success as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_1() {
        assert_eq!(Solution::ways_to_partition(vec![2, -1, 2], 3), 1)
    }

    #[test]
    fn testcase_2() {
        assert_eq!(Solution::ways_to_partition(vec![0, 0, 0], 1), 2)
    }

    #[test]
    fn testcase_3() {
        assert_eq!(
            Solution::ways_to_partition(
                vec![22, 4, -25, -20, -15, 15, -16, 7, 19, -10, 0, -13, -14],
                -33
            ),
            4
        )
    }

    #[test]
    fn testcase_4() {
        assert_eq!(
            Solution::ways_to_partition(
                vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30827, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0
                ],
                0
            ),
            33
        )
    }
}
