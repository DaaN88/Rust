fn main() {
    assert_eq!(find_duplicate(vec![1,1]), 1);
    assert_eq!(find_duplicate(vec![1,2,1]), 1);
    assert_eq!(find_duplicate(vec![1,3,4,2,2]), 2);
    assert_eq!(find_duplicate(vec![1,3,4,2,2]), 2);
    assert_eq!(find_duplicate(vec![3,1,3,4,2]), 3);
    assert_eq!(find_duplicate(vec![3,3,3,3,3]), 3);
    assert_eq!(find_duplicate(vec![1,2,3,4,5]), -1);
    //assert_eq!(find_duplicate(vec![11,3,0,5,3,23]), 3); // <- break; violated the conditions
}

pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    let mut slow = nums[0];
    let mut fast = nums[0];

    loop {
        slow = nums[slow as usize];

        fast = nums[nums[fast as usize] as usize];

        if slow == fast {
            break;
        }
    }

    slow = nums[0];

    while slow != fast {
        slow = nums[slow as usize];
        fast = nums[fast as usize];
    }

    slow
}
