pub struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        if s.len() == 0 || s.len() == 1 {
            return 0;
        }

        let mut count = 0;

        let mut vec: Vec<i32> = vec![-1];

        for (index, current_parenthesis) in s.chars().enumerate() {
            if current_parenthesis == '(' {
                vec.push(index as i32);
            }

            if current_parenthesis == ')' {
                vec.pop();

                if vec.len() == 0 {
                    vec.push(index as i32);
                } else {
                    count = count.max(index as i32 - vec[vec.len() - 1]);
                }
            }
        }
        count
    }
}

fn main() {
    assert_eq!(Solution::longest_valid_parentheses("".to_string()), 0);

    assert_eq!(Solution::longest_valid_parentheses("(".to_string()), 0);

    assert_eq!(Solution::longest_valid_parentheses(")".to_string()), 0);

    assert_eq!(Solution::longest_valid_parentheses("(()".to_string()), 2);

    assert_eq!(Solution::longest_valid_parentheses(")()())".to_string()), 4);

    assert_eq!(Solution::longest_valid_parentheses(")(())())".to_string()), 6);

    assert_eq!(Solution::longest_valid_parentheses("()(())".to_string()), 6);

    assert_eq!(Solution::longest_valid_parentheses("()(()".to_string()), 2);

    assert_eq!(Solution::longest_valid_parentheses(")()())()()(".to_string()), 4);

    assert_eq!(Solution::longest_valid_parentheses(")()()()()()(".to_string()), 10);

    assert_eq!(Solution::longest_valid_parentheses("(()))())(".to_string()), 4);

    assert_eq!(Solution::longest_valid_parentheses("(())((((())))(".to_string()), 8);
}
