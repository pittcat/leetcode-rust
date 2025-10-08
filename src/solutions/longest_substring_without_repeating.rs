use std::{cmp::max, collections::HashMap};
// ============================================================================
// 第 0 步:导入必要的依赖
// ============================================================================
// TODO: 在文件最顶部添加:
// use std::collections::HashMap;
// use std::cmp::max;

pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        // ====================================================================
        // 第 1 步:变量声明和初始化
        // ====================================================================
        // TODO: 声明以下变量(记得加 mut,因为它们会被修改):
        //
        // 1. 创建一个 HashMap 来存储字符和它的索引
        //    - 键类型: char (字符)
        //    - 值类型: usize (索引)
        //    - 语法: let mut map = HashMap::new();
        let mut map: HashMap<char, usize> = HashMap::new();

        //
        // 2. 创建 left 指针
        //    - 类型: usize (用于索引,不是 i32!)
        //    - 初始值: 0
        //    - 语法: let mut left = 0;

        let mut left: usize = 0;

        // 3. 创建 max_len 记录最大长度
        //    - 类型: usize
        //    - 初始值: 0
        //    - 语法: let mut max_len = 0;
        let mut max_len: usize = 0;
        // 你的代码写在这里 ↓

        // ====================================================================
        // 第 2 步:处理边界情况(可选但推荐)
        // ====================================================================
        // TODO: 检查空字符串
        // 如果 s.len() == 0,可以直接 return 0;
        // 这样可以避免后续不必要的计算
        if s.len() == 0 {
            return 0;
        }

        // 你的代码写在这里 ↓

        // ====================================================================
        // 第 3 步:遍历字符串的每个字符
        // ====================================================================
        // TODO: 使用 for 循环遍历字符串
        //
        // 语法: for (right, c) in s.chars().enumerate() {
        //     // right 是当前字符的索引(自动获得)
        //     // c 是当前字符
        // }
        // 为什么用 .chars().enumerate()?
        // - .chars() 返回字符迭代器(处理 UTF-8)
        // - .enumerate() 同时给出索引和值

        // 你的代码写在这里 ↓
        for (right, char_value) in s.chars().enumerate() {
            // ================================================================
            // 第 4 步:检测重复字符
            // ================================================================
            // TODO: 检查字符 char_value 是否在当前窗口内已经出现过
            //
            // 思路:
            // 1. 使用 map.get(&char_value) 查询字符
            // 2. 如果返回 Some(旧索引) 且 旧索引 >= left
            //    说明在当前窗口内发现了重复
            //
            // 推荐语法:
            // if let Some(&prev_index) = map.get(&char_value) {
            //     if prev_index >= left {
            //         // 发现重复,执行第 5 步
            //     }
            // }

            // 你的代码写在这里 ↓
            if let Some(&prev_index) = map.get(&char_value) {
                // ============================================================
                // 第 5 步:收缩窗口(处理重复)
                // ============================================================
                // TODO: 当发现重复字符时,移动 left 指针
                //
                // 语法: left = prev_index + 1;
                //
                // 为什么要检查 prev_index >= left?
                // - 因为 HashMap 存储的是字符的历史位置
                // - 可能这个字符在 left 左边已经被"抛弃"了
                // - 只有当它在当前窗口内(>= left)才需要收缩

                // 你的代码写在这里 ↓
                if prev_index >= left {
                    left = prev_index + 1;
                }
            }

            // ================================================================
            // 第 6 步:更新 HashMap
            // ================================================================
            // TODO: 更新字符 char_value 的最新位置
            //
            // 语法: map.insert(char_value, right);

            // 你的代码写在这里 ↓
            map.insert(char_value, right);

            // ================================================================
            // 第 7 步:更新最大长度
            // ================================================================
            // TODO: 计算当前窗口长度并更新 max_len
            //
            // 当前窗口长度 = right - left + 1
            //
            // 方式 A(推荐):
            // max_len = max(max_len, right - left + 1);
            max_len = max(max_len, right - left + 1);
            //
            // ⚠️ 注意:需要先在文件顶部添加 use std::cmp::max;
            //
            // 方式 B(不需要导入 max):
            // let current_len = right - left + 1;
            // if current_len > max_len {
            //     max_len = current_len;
            // }

            // 你的代码写在这里 ↓
        } // for 循环结束

        // ====================================================================
        // 第 8 步:返回结果
        // ====================================================================
        // TODO: 将 max_len 转换为 i32 并返回
        //
        // 为什么需要转换?
        // - max_len 是 usize 类型
        // - LeetCode 要求返回 i32
        //
        // 语法: max_len as i32
        //
        // as 转换是否安全?
        // - 题目限制: 0 <= s.length <= 50000
        // - i32 最大值: 2147483647
        // - 所以这里转换是安全的

        // 你的代码写在这里 ↓
        max_len as i32
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
