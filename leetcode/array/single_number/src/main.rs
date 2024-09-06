fn main() {
    assert_eq!(single_number(vec![2,2,1]), 1);
    assert_eq!(single_number(vec![4,1,2,1,2]), 4);
    assert_eq!(single_number(vec![1,1,2,4,2]), 4);
    assert_eq!(single_number(vec![1]), 1);
}

pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut target_num: i32 = 0;

    for num in nums {
        target_num ^= num;
    }

    target_num
}