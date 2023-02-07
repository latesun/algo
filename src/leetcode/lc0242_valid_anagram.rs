struct Solution {}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut s_chars = [0; 26];
        let mut t_chars = [0; 26];

        for char in s.chars() {
            s_chars[(char as usize) - 97] += 1;
        }

        for char in t.chars() {
            t_chars[(char as usize) - 97] += 1;
        }

        for i in 0..s_chars.len() {
            if s_chars[i] != t_chars[i] {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_anagram() {
        assert!(Solution::is_anagram(
            "anagram".to_string(),
            "nagaram".to_string()
        ));
        assert!(!Solution::is_anagram("rat".to_string(), "car".to_string()));
        assert!(!Solution::is_anagram("a".to_string(), "ab".to_string()));
    }
}
