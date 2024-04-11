pub struct Solution;

impl Solution {
    pub fn minimum_xor_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mask = (0..nums1.len()).collect::<Vec<_>>();
        let mut cache = std::collections::HashMap::new();
        Solution::dp(&nums1, &nums2, &mask, &mut cache)
    }

    fn dp(
        nums1: &[i32],
        nums2: &[i32],
        mask: &[usize],
        cache: &mut std::collections::HashMap<usize, i32>,
    ) -> i32 {
        if mask.is_empty() {
            return 0;
        }
        let h = mask.iter().fold(0, |acc, idx| acc + (1 << idx));
        if let Some(&v) = cache.get(&h) {
            return v;
        }
        let mut res = std::i32::MAX;
        for (idx, m) in mask.iter().enumerate() {
            let new_mask = [&mask[..idx], &mask[idx + 1..]].concat();
            res = res.min(
                (nums1[*m] ^ nums2[new_mask.len()]) + Solution::dp(nums1, nums2, &new_mask, cache),
            );
        }
        cache.insert(h, res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_1() {
        assert_eq!(Solution::minimum_xor_sum(vec![1, 2], vec![2, 3]), 2)
    }

    #[test]
    fn testcase_2() {
        assert_eq!(Solution::minimum_xor_sum(vec![1, 0, 3], vec![5, 3, 4]), 8)
    }

    #[test]
    fn testcase_3() {
        assert_eq!(
            Solution::minimum_xor_sum(vec![72, 97, 8, 32, 15], vec![63, 97, 57, 60, 83]),
            152
        )
    }

    #[test]
    fn testcase_4() {
        assert_eq!(
            Solution::minimum_xor_sum(
                vec![100, 26, 12, 62, 3, 49, 55, 77, 97],
                vec![98, 0, 89, 57, 34, 92, 29, 75, 13]
            ),
            200
        )
    }
}
