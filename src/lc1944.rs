pub struct Solution;

impl Solution {
    pub fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
        let mut desc_stack = Vec::new();
        let mut result = vec![0; heights.len()];
        for (i, &h) in heights.iter().enumerate().rev() {
            while let Some(&next) = desc_stack.last() {
                if next > h {
                    break;
                }
                desc_stack.pop();
                // See everyone in between that are not blocked
                result[i] += 1;
            }
            if desc_stack.len() > 0 {
                // See the taller person
                result[i] += 1;
            }
            desc_stack.push(h);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_1() {
        assert_eq!(
            Solution::can_see_persons_count(vec![10, 6, 8, 5, 11, 9]),
            vec![3, 1, 2, 1, 1, 0]
        )
    }

    #[test]
    fn testcase_2() {
        assert_eq!(
            Solution::can_see_persons_count(vec![5, 1, 2, 3, 10]),
            vec![4, 1, 1, 1, 0]
        )
    }
}
