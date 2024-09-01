use std::collections::HashMap;

fn main() {
    println!("res: {}", missing_number(vec![0,1]));
}

pub fn missing_number(nums: Vec<i32>) -> i32 {
    let mut elems: HashMap<i32, i32> = HashMap::new();

    let max_num = nums.iter().max().unwrap();
    let nums_len = nums.len() as i32;

    let mut quantity_num = max_num;

    if max_num < &nums_len {
        quantity_num = &nums_len;
    }

    for elem in 0..quantity_num + 1 {
        elems.insert(elem, 0);
    }

    for num in nums.iter() {
        if elems.contains_key(&num) {
            let current_num_val = elems.get(num).unwrap();

            let countable_num_val = current_num_val + 1;

            elems.insert(*num, countable_num_val);
        }
    }

    for (num, num_val) in &elems {
        if num_val == &0 {
            return *num;
        }
    }

    0
}
