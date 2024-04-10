struct Solution;

impl Solution {
    pub fn minimum_white_tiles(floor: String, num_carpets: i32, carpet_len: i32) -> i32 {
        let num_carpets = num_carpets as usize;
        let carpet_len = carpet_len as usize;
        let floor = floor
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>();
        // dp[i] is the minimum white tiles left for the first i tiles with num_iter of carpets
        let mut dp = std::iter::once(&0)
            .chain(floor.iter())
            .scan(0, |acc, &x| {
                *acc += x;
                Some(*acc)
            })
            .collect::<Vec<_>>();
        for _ in 0..num_carpets {
            let mut new_dp = dp.clone();
            for i in 1..carpet_len {
                new_dp[i] = 0;
            }
            for i in carpet_len..=floor.len() {
                // If we put a carpet here, the white tiles will be covered
                // Otherwise, the same amount of white tiles as the previous state + the current tile
                new_dp[i] = dp[i - carpet_len].min(new_dp[i - 1] + floor[i - 1]);
            }
            dp = new_dp;
        }
        *dp.last().unwrap() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_1() {
        assert_eq!(
            Solution::minimum_white_tiles(String::from("10110101"), 2, 2),
            2
        )
    }

    #[test]
    fn testcase_2() {
        assert_eq!(
            Solution::minimum_white_tiles(String::from("11111"), 2, 3),
            0
        )
    }

    #[test]
    fn testcase_3() {
        assert_eq!(
            Solution::minimum_white_tiles(
                String::from("1000000000001000000100111100001101111000000001001111110000000000"),
                6,
                4
            ),
            3
        )
    }

    #[test]
    fn testcase_4() {
        assert_eq!(
            Solution::minimum_white_tiles(
                String::from("1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111"),
                33,
                34
            ),
            0
        )
    }
}
