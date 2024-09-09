fn main() {
    assert_eq!(find_duplicates(vec![1]), Vec::new());
    assert_eq!(find_duplicates(vec![1,1]), vec![1]);

    let mut vec_1 = find_duplicates(vec![4,3,2,7,8,2,3,1]);
    vec_1.sort();

    assert_eq!(vec_1, vec![2,3]);
}

pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
    let mut duplicates: Vec<i32> = Vec::new();

    for i in 0..nums.len() {
        let idx = (nums[i].abs() - 1) as usize;

        nums[idx] *= -1;

        if nums[idx] > 0 {
            duplicates.push(nums[i].abs());
        }
    }

    return duplicates;
}
