fn main() {
    assert_eq!(product_except_self(vec![1,2,3,4]), vec![24,12,8,6]);
    assert_eq!(product_except_self(vec![-1,1,0,-3,3]), vec![0,0,9,0,0]);
}

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut results = vec![1; nums.len()];

    let mut left_pointer = 0;
    let mut right_pointer = nums.len()-1;

    let mut left_val = 1;
    let mut right_val = 1;

    loop {
        results[left_pointer] = results[left_pointer] * left_val;
        results[right_pointer] = results[right_pointer] * right_val;

        right_val = right_val * nums[right_pointer];
        left_val = left_val * nums[left_pointer];

        if right_pointer == 0 {
            break
        }

        left_pointer += 1;
        right_pointer -= 1;
    }

    results
}
