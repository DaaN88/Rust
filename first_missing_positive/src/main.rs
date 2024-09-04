fn main() {
    assert_eq!(first_missing_positive(vec![1,2,0]), 3);
    assert_eq!(first_missing_positive(vec![3,4,-1,1]), 2);
    assert_eq!(first_missing_positive(vec![3,2,4,-1,1]), 5);
    assert_eq!(first_missing_positive(vec![7,8,9,11,12]), 1);
    assert_eq!(first_missing_positive(vec![7,8,9,11,12,1]), 2);
    assert_eq!(first_missing_positive(vec![7,8,2,9,1,12]), 3);
    assert_eq!(first_missing_positive(vec![0]), 1);
    assert_eq!(first_missing_positive(vec![-5]), 1);
}

pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
    let nums_len = nums.len();

    let mut nums = nums;

    for n in &mut nums {
        if *n <= 0 {
            *n = i32::MAX;
        }
    }

    for index in 0..nums_len {
        let n = (nums[index].abs() - 1) as usize;

        if n >= nums_len {
            continue;
        }

        nums[n] = -nums[n].abs();
    }

    (nums.into_iter().position(|num| num > 0).unwrap_or(nums_len) + 1) as i32
}