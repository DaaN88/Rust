pub fn binary_search(data: &[i32], value: i32) -> usize {
    let mut idx = 1;

    while idx < data.len() {
        let el = data[idx];

        if el == value {
            return idx;
        }
        
        idx = 2 * idx + usize::from(el < value);
    }

    0
  }

// =====================================================================================================================

fn main() {
    println!("Sort numbers ascending");
    let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];

    println!("Numbers: {:?}", numbers);

    let result = binary_search(&mut numbers, 10);

    println!("Result [index = 0 if value not found]:  {:?}\n", result);
}
