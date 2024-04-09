struct Solution;

impl Solution {
    pub fn min_cost(n: i32, cuts: Vec<i32>) -> i32 {
        let mut cuts = cuts;
        cuts.sort();
        let mut segments = vec![0];
        segments.extend(cuts.to_owned());
        segments.push(n);
        let mut segments = segments.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
        let mut cost = 0;
        while cuts.len() > 0 {
            let (w, idx) = segments
                .windows(2)
                .zip(0..)
                .min_by(|(a, _), (c, _)| (a[0] + a[1]).cmp(&(c[0] + c[1])))
                .unwrap();
            cost += w[0] + w[1];
            segments[idx] += segments[idx + 1];
            segments.remove(idx + 1);
            cuts.remove(idx);
        }
        cost
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
}
