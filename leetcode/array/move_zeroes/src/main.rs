fn main() {
    let vec = &mut vec![0,1,0,3,12];

    move_zeroes(vec);

    assert_eq!(vec, &vec![1,3,12,0,0]);

    let vec_1 = &mut vec![0,1,0,3,12,0];

    move_zeroes(vec_1);

    assert_eq!(vec_1, &vec![1,3,12,0,0,0]);
}

pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut right_pointer;

    let nums_end_pointer = nums.len() - 1;

    for left_pointer in 0..=nums_end_pointer {
        right_pointer = left_pointer;

        if nums[left_pointer] == 0 {
            while nums[right_pointer] == 0 {
                if right_pointer == nums_end_pointer {
                    break
                }

                right_pointer += 1;
            }

            nums.swap(left_pointer, right_pointer);
        }

        let mut control_sum = 0;
        let mut tmp_rp = right_pointer;

        while control_sum == 0 {
            control_sum += nums[tmp_rp];

            if tmp_rp == nums_end_pointer {
                break
            }

            tmp_rp += 1;
        }

        if control_sum == 0 {
            break;
        }
    }
}