pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let result: i32 = 0;
        let left: i32 = 0;
        let right: i32 = 0;
        let max_len = 0;

        result
    }
}

// ============================================================================
// 测试用例
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(Solution::length_of_longest_substring("".to_string()), 0);
    }

    #[test]
    fn test_single_char() {
        assert_eq!(Solution::length_of_longest_substring("a".to_string()), 1);
    }

    #[test]
    fn test_all_unique() {
        assert_eq!(
            Solution::length_of_longest_substring("abcdef".to_string()),
            6
        );
    }

    #[test]
    fn test_with_spaces() {
        assert_eq!(
            Solution::length_of_longest_substring("a b c a".to_string()),
            3
        );
    }
}
