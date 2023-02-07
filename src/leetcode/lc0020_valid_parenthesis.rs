struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = "".to_string();
        for char in s.chars() {
            match char {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                ')' | ']' | '}' => {
                    if stack.pop() != Some(char) {
                        return false;
                    }
                }
                _ => (),
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid() {
        assert!(Solution::is_valid("()".to_string()));
        assert!(Solution::is_valid("()[]{}".to_string()));
        assert!(!Solution::is_valid("(]".to_string()));
    }
}
