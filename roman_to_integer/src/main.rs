pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        s.chars().rfold(0, |acc, c| {
            acc + match c {
                'I' if acc >= 5 => -1,
                'I' => 1,
                'V' => 5,
                'X' if acc >= 50 => -10,
                'X' => 10,
                'L' => 50,
                'C' if acc >= 500 => -100,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            }
        })
    }
}

fn main() {
    assert_eq!(Solution::roman_to_int("I".to_string()), 1);

    assert_eq!(Solution::roman_to_int("V".to_string()), 5);

    assert_eq!(Solution::roman_to_int("X".to_string()), 10);

    assert_eq!(Solution::roman_to_int("L".to_string()), 50);

    assert_eq!(Solution::roman_to_int("C".to_string()), 100);

    assert_eq!(Solution::roman_to_int("D".to_string()), 500);

    assert_eq!(Solution::roman_to_int("M".to_string()), 1000);
}
