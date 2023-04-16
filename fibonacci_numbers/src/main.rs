fn main() {
    println!("fibonacci number: {}", fib(25));
    println!("fibonacci (tail) number: {}", fib_tail(15));
}

// =================================================================================================

fn fib(number: i64) -> i64 {
    if number == 0 { return 0 }
    if number == 1 { return 1 }

    return fib (number - 1) + fib(number - 2)
}

// ====== TAIL RECURSION ===========================================================================

// For a tail call to remove stack frames, it must contain all the parameters in
// the tail call function itself
fn fib_tail(number: i64) -> i64 {
    fn fib_tail_inner(first_num: i64, second_num: i64, number: i64) -> i64 {
        if number == 0 { return first_num }

        return fib_tail_inner(second_num, first_num + second_num, number - 1)
    }

    return fib_tail_inner(0, 1, number)
}
