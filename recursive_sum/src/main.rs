fn main() {
    let numbers = vec![1, 2, 3, 4];

    println!("Recursive sum on slice");
    println!("Result: {:?}\n", recursive_sum_on_slice(&numbers));

    println!("Recursive sum on slice directly");
    println!("Result: {:?}\n", recursive_sum_on_slice_directly(&numbers));

    println!("Recursive sum tail recursion");
    println!("Result: {:?}\n", sum_tail_recursion(&numbers, 0));
}

// =====================================================================================================================

pub fn recursive_sum_on_slice(numbers: &Vec<usize>) -> usize {
    match numbers.as_slice() {
        [] => 0,

        [first, rest @ ..] => {
            // [to_vec] performs a heap allocation, so calling [recursive_sum_on_slice] recursively is very inefficient
            first + recursive_sum_on_slice(&rest.to_vec())
        }
    }
}

pub fn recursive_sum_on_slice_directly(numbers: &[usize]) -> usize {
    match numbers {
        [] => 0,

        [first, rest @ ..] => {
            first + recursive_sum_on_slice_directly(&rest)
        }
    }
}

pub fn sum_tail_recursion(numbers: &[usize], accum: usize) -> usize {
    match numbers {
        [] => accum,

        [first, rest @ ..] => sum_tail_recursion(rest, accum + first)
    }
}
