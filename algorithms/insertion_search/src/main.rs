pub fn insertion_sort<T: std::cmp::PartialOrd>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;

        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j-1);

            j = j - 1;
        }
    }
}

// =====================================================================================================================

fn main() {
    println!("Sort numbers ascending");
    let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];

    println!("Before: {:?}", numbers);
    insertion_sort(&mut numbers);
    println!("After:  {:?}\n", numbers);

    println!("Sort strings alphabetically");
    let mut strings = ["beach", "hotel", "airplane", "car", "house", "art"];

    println!("Before: {:?}", strings);
    insertion_sort(&mut strings);
    println!("After:  {:?}\n", strings);

    println!("Sort date as strings (without time)");
    let mut strings = ["2023-01-01", "2023-01-02", "2023-01-04", "2022-01-01", "2023-01-05", "2023-01-11"];

    println!("Before: {:?}", strings);
    insertion_sort(&mut strings);
    println!("After:  {:?}\n", strings);
}