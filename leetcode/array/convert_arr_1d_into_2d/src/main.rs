fn main() {
    assert_eq!(construct2_d_array(vec![1,2,3,4], 2, 2), vec![vec![1,2], vec![3,4]]);
    assert_eq!(construct2_d_array(vec![1,2,3], 1, 3), vec![vec![1,2,3]]);
    assert_eq!(construct2_d_array(vec![1,2], 1, 1),  Vec::<Vec<i32>>::new());
    assert_eq!(construct2_d_array(vec![5], 3, 1),  Vec::<Vec<i32>>::new());
}

pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
    let mut vecs_result: Vec<Vec<i32>> = Vec::<Vec<i32>>::new();

    if m * n != original.len() as i32 {
        return vec![];
    }

    let mut tmp_vec: &mut Vec<i32> = &mut Vec::<i32>::new();

    for (index, num) in original.iter().enumerate() {
        tmp_vec.push(*num);

        if (index as i32 + 1) % n == 0 {
            vecs_result.push(tmp_vec.to_vec());

            tmp_vec.clear();
        }
    }

    vecs_result
}
