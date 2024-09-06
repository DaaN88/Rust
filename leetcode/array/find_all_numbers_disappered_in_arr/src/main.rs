fn main() {
    assert_eq!(find_disappeared_numbers(vec![4,3,2,7,8,2,3,1]), vec![5,6]);
    assert_eq!(find_disappeared_numbers(vec![1,1]), vec![2]);
}

pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
    let upper_end_range = nums.len();

    let mut result: Vec<i32> = Vec::<i32>::new();

    for elem in 1..upper_end_range + 1 {
        if ! nums.contains(&(elem as i32)) {
            result.push(elem as i32);
        }
    }

    result
}
