Given a string containing just the characters '(' and ')', return the length of the longest valid (well-formed) parentheses 
substring.

Example 1:

Input: s = "(()"
Output: 2
Explanation: The longest valid parentheses substring is "()".
Example 2:

Input: s = ")()())"
Output: 4
Explanation: The longest valid parentheses substring is "()()".
Example 3:

Input: s = ""
Output: 0


Constraints:

0 <= s.length <= 3 * 104
s[i] is '(', or ')'.

Approach
Initialize a stack and push -1 onto it. The stack will store the indices of left parentheses '(' in the input string.
Iterate through the characters of the input string, and for each character:
If it's a left parenthesis, push its index onto the stack.
If it's a right parenthesis:
Pop the top index from the stack. If the stack is not empty, the length of the current valid parentheses substring is the difference between the current index and the index at the top of the stack. If the stack is empty, push the current index onto the stack.
Return the maximum length of valid parentheses substring found.