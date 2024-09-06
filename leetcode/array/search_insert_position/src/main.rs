use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut begin_index = 0;
        let mut last_index = nums.len() - 1;

        if target < nums[0] {
            return 0;
        }

        // binary search
        while begin_index <= last_index {
            let middle_index = (begin_index + last_index) / 2;

            match target.cmp(&nums[middle_index]) {
                Ordering::Equal => return middle_index as i32,
                Ordering::Less => last_index = middle_index - 1,
                Ordering::Greater => begin_index = middle_index + 1
            }
        }

        begin_index as i32
    }
}

fn main() {
    assert_eq!(Solution::search_insert(vec![1,3,5,6], 5), 2);

    assert_eq!(Solution::search_insert(vec![1,3,5,6], 2), 1);

    assert_eq!(Solution::search_insert(vec![1,3,5,6], 7), 4);

    assert_eq!(Solution::search_insert(vec![1,3,5,6], 0), 0);

    assert_eq!(Solution::search_insert(vec![2,5], 1), 0);

    assert_eq!(Solution::search_insert(vec![-1,3,5,6], 0), 1);
}
