struct Solution;

impl Solution {
    pub fn min_cost(n: i32, cuts: Vec<i32>) -> i32 {
        let mut cuts = cuts;
        cuts.sort();
        cuts.insert(0, 0);
        cuts.push(n);
        let n = cuts.len();
        let mut dp = vec![vec![std::i32::MAX; n]; n];
        for i in 0..cuts.len() - 1 {
            dp[i][i + 1] = 0;
        }
        for num_segs in 2..n {
            for (i, j) in (0..n).zip(num_segs..n) {
                let step_cost = cuts[j] - cuts[i];
                dp[i][j] = dp[i][j]
                    .min((i + 1..j).map(|k| dp[i][k] + dp[k][j]).min().unwrap_or(0) + step_cost);
            }
        }
        dp[0][cuts.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_1() {
        assert_eq!(Solution::min_cost(7, vec![1, 3, 4, 5]), 16)
    }

    #[test]
    fn testcase_2() {
        assert_eq!(Solution::min_cost(9, vec![5, 6, 1, 4, 2]), 22)
    }

    #[test]
    fn testcase_3() {
        assert_eq!(
            Solution::min_cost(
                36,
                vec![13, 17, 15, 18, 3, 22, 27, 6, 35, 7, 11, 28, 26, 20, 4, 5, 21, 10, 8]
            ),
            150
        )
    }
}
