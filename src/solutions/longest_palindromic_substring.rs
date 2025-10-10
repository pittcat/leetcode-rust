// ============================================================================
// LeetCode 5: 最长回文子串 (Longest Palindromic Substring)
// ============================================================================
// 难度: 中等
// 链接: https://leetcode.cn/problems/longest-palindromic-substring/
//
// 题目描述:
// 给你一个字符串 s,找到 s 中最长的回文子串。
//
// 示例:
// 输入: s = "babad"
// 输出: "bab" (或 "aba")
//
// 输入: s = "cbbd"
// 输出: "bb"
//
// 约束条件:
// - 1 <= s.length <= 1000
// - s 仅由数字和英文字母组成
// ============================================================================

pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        // ====================================================================
        // 第 1 步: 边界检查
        // ====================================================================
        // TODO: 处理特殊情况
        // - 如果字符串长度 < 2,直接返回 s
        // - 单字符或空字符串本身就是回文

        // 你的代码写在这里 ↓
        if s.len() < 2 {
            return s;
        }

        // ====================================================================
        // 第 2 步: 字符串预处理
        // ====================================================================
        // TODO: 将 String 转换为 Vec<char>
        //
        // 为什么需要转换?
        // - Rust 的 String 是 UTF-8 编码,不能直接用 s[i] 访问
        // - Vec<char> 允许 O(1) 时间按索引访问字符
        //
        // 语法: let chars: Vec<char> = s.chars().collect();
        let chars: Vec<char> = s.chars().collect();

        // 你的代码写在这里 ↓

        // ====================================================================
        // 第 3 步: 初始化变量
        // ====================================================================
        // TODO: 声明以下变量:
        //
        // 1. start: usize = 0
        //    - 记录最长回文的起始索引
        //
        // 2. max_len: usize = 1
        //    - 记录最长回文的长度
        //    - 初始值为 1 (至少有单个字符)

        // 你的代码写在这里 ↓
        let mut start: usize = 0;
        let mut max_len: usize = 1;

        // ====================================================================
        // 第 4 步: 遍历每个可能的回文中心
        // ====================================================================
        // TODO: 使用 for 循环遍历每个位置
        //
        // for i in 0..chars.len() {
        //     // 情况 1: 奇数长度回文 (中心是单个字符)
        //     // 情况 2: 偶数长度回文 (中心是两个字符)
        // }

        // 你的代码写在这里 ↓

        for i in 0..chars.len() {
            // ================================================================
            // 第 5 步: 处理奇数长度回文
            // ================================================================
            // TODO: 调用辅助函数扩展
            //
            //
            // 为什么传 i, i?
            // - 中心是单个字符 chars[i]
            // - 从 i 向两边扩展

            // ================================================================
            // 第 6 步: 处理偶数长度回文
            // ================================================================
            // TODO: 调用辅助函数扩展
            //
            // let len2 = Self::expand_around_center(&chars, i as i32, (i + 1) as i32);
            //
            // 为什么传 i, i+1?
            // - 中心是两个字符 chars[i] 和 chars[i+1]
            // - 从 i, i+1 向两边扩展

            // 你的代码写在这里 ↓
            let len1 = Self::expand_around_center(&chars, i as i32, i as i32);
            let len2 = Self::expand_around_center(&chars, i as i32, (i + 1) as i32);
            // ================================================================
            // 第 7 步: 更新最长回文记录
            // ================================================================
            // TODO: 取两种情况的最大值
            //
            let len = len1.max(len2);

            if len > max_len {
                max_len = len;
                start = i - (len - 1) / 2;
            }

            //
            // 如果 len > max_len:
            //     max_len = len;
            //     start = i - (len - 1) / 2;
            //
            // 为什么 start = i - (len - 1) / 2?
            // - i 是中心位置
            // - 向左偏移 (len - 1) / 2 就是起始位置
            //
            // 例如: "babad", i=2 ('b'), len=3
            // start = 2 - (3-1)/2 = 2 - 1 = 1
            // 回文是 s[1..4] = "aba"

            // 你的代码写在这里 ↓
        }
        // ====================================================================
        // 第 8 步: 提取并返回结果
        // ====================================================================
        // TODO: 从 chars 中提取子串
        //
        // 方式 A: 从 Vec<char> 提取
        // chars[start..start+max_len].iter().collect()

        //
        // 方式 B: 从原字符串提取 (如果保留了 s)
        // s[start..start+max_len].to_string()
        //
        // ⚠️ 注意: 如果用方式 B,需要先确保没有移动 s 的所有权

        // 你的代码写在这里 ↓
        s[start..start + max_len].to_string()
    }

    // ========================================================================
    // 辅助函数: 从中心向两边扩展
    // ========================================================================
    // 功能: 给定中心位置 left, right,向两边扩展找最长回文
    // 参数:
    //   - chars: &[char] - 字符数组的切片(借用)
    //   - left: i32 - 左边界(可能为负)
    //   - right: i32 - 右边界(可能越界)
    // 返回: usize - 回文长度
    fn expand_around_center(chars: &[char], mut left: i32, mut right: i32) -> usize {
        // ====================================================================
        // 第 9 步: 实现扩展逻辑
        // ====================================================================
        // TODO: 当条件满足时,向两边扩展
        //
        // 条件:
        // 1. left >= 0 (左边界不越界)
        // 2. right < chars.len() as i32 (右边界不越界)
        // 3. chars[left as usize] == chars[right as usize] (两边字符相同)
        //
        // 循环体:
        //     left -= 1;
        //     right += 1;
        //
        // 循环结束后:
        //     返回 (right - left - 1) as usize
        //
        // 为什么是 right - left - 1?
        // - 循环结束时 left, right 已经越过回文边界一步
        // - 实际长度 = right - left - 1
        //
        // 例如: "aba"
        // 初始: left=1, right=1
        // 第 1 次: left=0, right=2 (chars[0]='a', chars[2]='a' 相等)
        // 第 2 次: left=-1, right=3 (越界,退出)
        // 长度 = 3 - (-1) - 1 = 3 ✓

        // 你的代码写在这里 ↓
        while left >= 0
            && right < chars.len() as i32
            && chars[left as usize] == chars[right as usize]
        {
            left -= 1;
            right += 1;
        }

        (right - left - 1) as usize
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
        let result = Solution::longest_palindrome("babad".to_string());
        // "bab" 和 "aba" 都是正确答案
        assert!(result == "bab" || result == "aba");
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb");
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::longest_palindrome("abb".to_string()), "bb");
    }

    #[test]
    fn test_single_char() {
        assert_eq!(Solution::longest_palindrome("a".to_string()), "a");
    }

    #[test]
    fn test_all_same() {
        assert_eq!(Solution::longest_palindrome("aaaa".to_string()), "aaaa");
    }

    #[test]
    fn test_no_palindrome_except_single() {
        let result = Solution::longest_palindrome("abcdef".to_string());
        assert_eq!(result.len(), 1); // 任意单个字符
    }

    #[test]
    fn test_even_palindrome() {
        assert_eq!(Solution::longest_palindrome("abba".to_string()), "abba");
    }

    #[test]
    fn test_odd_palindrome() {
        assert_eq!(
            Solution::longest_palindrome("racecar".to_string()),
            "racecar"
        );
    }
}
