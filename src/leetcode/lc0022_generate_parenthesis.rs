struct Solution {}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if n < 1 {
            return vec![];
        }

        let mut parentheses: Vec<String> = Vec::new();
        Self::generate(0, 0, n, "".to_string(), &mut parentheses);
        parentheses
    }

    #[allow(clippy::redundant_clone)]
    fn generate(left: i32, right: i32, n: i32, parenthesis: String, parentheses: &mut Vec<String>) {
        if left == n && right == n {
            parentheses.push(parenthesis);
            return;
        }

        if left < n {
            let mut parenthesis = parenthesis.clone();
            parenthesis.push('(');
            Self::generate(left + 1, right, n, parenthesis, parentheses);
        }

        if right < left {
            let mut parenthesis = parenthesis.clone();
            parenthesis.push(')');
            Self::generate(left, right + 1, n, parenthesis, parentheses);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_parenthesis() {
        assert_eq!(Solution::generate_parenthesis(1), vec!["()"]);
        assert_eq!(Solution::generate_parenthesis(2), vec!["(())", "()()"]);
        assert_eq!(
            Solution::generate_parenthesis(3),
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
    }
}
