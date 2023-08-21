pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false
        }

        let mut number = x;

        let mut palindrome = 0_i32;

        while number > 0 {
            palindrome = palindrome * 10 + number % 10;

            number /= 10;
        }

        palindrome == x
    }
}

fn main() {
    assert_eq!(Solution::is_palindrome(727), true);

    assert_eq!(Solution::is_palindrome(121), true);

    assert_eq!(Solution::is_palindrome(1221), true);

    assert_eq!(Solution::is_palindrome(123421), false);

    assert_eq!(Solution::is_palindrome(-121), false);

    assert_eq!(Solution::is_palindrome(10), false);

    assert_eq!(Solution::is_palindrome(0), true);
}
