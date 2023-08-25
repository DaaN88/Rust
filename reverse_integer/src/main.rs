pub struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut number = x;

        let mut is_negative = false;

        if number < 0 {
            number *= -1;

            is_negative = true;
        }

        let mut reversed = 0_i32;

        while number > 0 {
            if reversed.checked_mul(10) == None {
                return 0;
            }

            reversed = reversed * 10 + number % 10;

            number /= 10;
        }

        if is_negative {
            return reversed as i32 * -1;
        }

        reversed as i32
    }
}

fn main() {
    assert_eq!(Solution::reverse(123), 321);

    assert_eq!(Solution::reverse(-123), -321);

    assert_eq!(Solution::reverse(120), 21);

    assert_eq!(Solution::reverse(1534236469), 0);
}
