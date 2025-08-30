pub struct Solution;

struct TrieNode {
    zero: Option<Box<TrieNode>>,
    one: Option<Box<TrieNode>>,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            zero: None,
            one: None,
        }
    }

    fn insert(&mut self, num: &i32) {
        let mut node = self;
        for bit in (0..32).rev().map(|n| (num >> n) & 1) {
            node = if bit == 0 {
                node.zero.get_or_insert_with(|| Box::new(TrieNode::new()))
            } else {
                node.one.get_or_insert_with(|| Box::new(TrieNode::new()))
            };
        }
    }

    fn get_one(&self) -> Option<&TrieNode> {
        self.one.as_deref()
    }

    fn get_zero(&self) -> Option<&TrieNode> {
        self.zero.as_deref()
    }
}

impl Solution {
    fn argsort<T: Ord>(data: &[T]) -> Vec<usize> {
        let mut indexed_data = data.iter().enumerate().collect::<Vec<_>>();
        indexed_data.sort_by_key(|&(_, value)| value);
        indexed_data.into_iter().map(|(index, _)| index).collect()
    }

    fn search(num: i32, options: &TrieNode) -> i32 {
        let mut node = options;
        let mut result = 0;
        for curr_bit in (0..32).rev().map(|n: i32| (num >> n) & 1) {
            let (better, worse) = if curr_bit == 0 {
                (node.get_one(), node.get_zero())
            } else {
                (node.get_zero(), node.get_one())
            };
            match (better, worse) {
                (Some(b), _) => {
                    result <<= 1;
                    result |= 1;
                    node = b;
                }
                (None, Some(w)) => {
                    result <<= 1;
                    result |= 0;
                    node = w;
                }
                _ => return -1,
            }
        }
        result
    }

    pub fn maximize_xor(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        // First filter out duplicates
        let nums = nums.into_iter().collect::<std::collections::HashSet<_>>();
        // Sort and insert into a binary heap
        let mut nums = nums
            .into_iter()
            .map(std::cmp::Reverse)
            .collect::<std::collections::BinaryHeap<_>>();

        // Temporarily sort queries for small values of m
        let indices = Self::argsort(&queries.iter().map(|q| q[1]).collect::<Vec<_>>());
        let mut results = vec![-1; queries.len()];

        // Construct a Trie as needed
        let mut root = TrieNode::new();

        // Process each query
        for idx in indices {
            let query = &queries[idx];
            let (x, m) = (query[0], query[1]);
            while let Some(std::cmp::Reverse(num)) = nums.peek()
                && num <= &m
            {
                root.insert(num);
                nums.pop();
            }
            // Find the maximum XOR for the current query
            results[idx] = Self::search(x, &root);
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_1() {
        assert_eq!(
            Solution::maximize_xor(
                vec![0, 1, 2, 3, 4],
                vec![vec![3, 1], vec![1, 3], vec![5, 6]]
            ),
            vec![3, 3, 7]
        )
    }

    #[test]
    fn testcase_2() {
        assert_eq!(
            Solution::maximize_xor(
                vec![5, 2, 4, 6, 6, 3],
                vec![vec![12, 4], vec![8, 1], vec![6, 3]]
            ),
            vec![15, -1, 5]
        )
    }
}
