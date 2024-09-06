use std::collections::HashMap;

fn main() {
    let vec = vec![1,2,3,4];

    println!("result: {}", has_duplicate(vec));
}

fn has_duplicate(nums: Vec<isize>) -> bool {
    let mut num_vals = HashMap::new();

    for num in nums.iter() {
        if ! num_vals.contains_key(num) {
            num_vals.insert(num, 0);
        } else {
            return true
        }
    }

    false
}
