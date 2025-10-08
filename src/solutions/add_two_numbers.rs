use core::slice;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy_head = Box::new(ListNode::new(0));
        let mut current = &mut dummy_head;
        let mut l1 = l1;
        let mut l2 = l2;
        let mut carry = 0;

        // 🎯 实现思路：
        //
        // 1. 【设置基础变量】
        //    - 创建一个哑节点 (dummy head) 简化链表构建
        //    - 设置可变引用 current 用于构建结果链表
        //    - 将 l1, l2 重新绑定为可变变量，方便移动
        //    - 初始化进位变量 carry = 0

        //
        // 2. 【主循环条件】
        //    - 当 l1.is_some() || l2.is_some() || carry != 0 时继续
        //    - 这样处理不同长度链表和最后的进位
        while l1.is_some() || l2.is_some() || carry != 0 {
            // 3. 【获取当前位数值】
            let x = if let Some(node) = l1 {
                let val = node.val;
                l1 = node.next; // ⭐ 移动到下一个节点
                val
            } else {
                0
            };

            let y = if let Some(node) = l2 {
                let val = node.val;
                l2 = node.next;
                val
            } else {
                0
            };

            // // 4. 【计算和进位】
            let sum = carry + x + y;
            carry = sum / 10; // 新的进位
            let digit = sum % 10; // 当前位数值
            //
            // // 5. 【构建结果节点】
            current.next = Some(Box::new(ListNode::new(digit)));
            current = current.next.as_mut().unwrap(); // 移动 current 指针
        }

        // 6. 【返回结果】
        dummy_head.next // 跳过哑节点
    }

    pub fn create_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
        // 1. 【边界检查】
        if nums.is_empty() {
            return None;
        }

        // 2. 【创建头节点】
        // 注意: 这里不用 Some 包装，直接创建 Box<ListNode>
        let mut head = Box::new(ListNode::new(nums[0]));

        // 3. 【获取可变引用】
        // current 是指向当前节点的可变引用
        let mut current = &mut head;

        // 4. 【构建链表】
        // 使用 if let 解构 Option
        if let Some(slice_nums) = nums.get(1..) {
            for &val in slice_nums {
                // 注意: &val 解构出 i32
                // 创建新节点并赋值给 current.next
                current.next = Some(Box::new(ListNode::new(val)));

                // 移动 current 指针到新创建的节点
                // as_mut() 将 &mut Option<Box<T>> 转换为 Option<&mut Box<T>>
                // unwrap() 提取 &mut Box<ListNode>
                //
                // 为什么必须用 unwrap() 而不能用 if let？
                // - if let 创建变量 next 持有借用，借用跨越整个 if 块
                // - 在 if 块内修改 current 会产生借用冲突（E0506）
                // - unwrap() 立即消费借用，借用在一行内结束，无冲突

                current = match current.next.as_mut() {
                    Some(next) => next,
                    None => unreachable!("New node"),
                }
            }
        }

        return Some(head);
    }

    // 辅助函数：将链表转换为Vec，用于测试和调试
    pub fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();

        while let Some(node) = head {
            result.push(node.val);
            head = node.next
        }

        result
    }
}

// ===========================================
// 测试模块解释
// ===========================================
// 这里演示Rust模块系统的概念

// 1. 普通模块 - 总是会被编译
mod helper_functions {
    pub fn format_list_debug(list: &Option<Box<super::ListNode>>) -> String {
        match list {
            Some(node) => format!("ListNode {{ val: {}, next: ... }}", node.val),
            None => "None".to_string(),
        }
    }
}

// 2. 条件编译测试模块 - 只在测试时编译
#[cfg(test)]
mod tests {
    use super::helper_functions::*;
    use super::*; // 导入父模块的所有公共项 // 导入helper_functions模块的公共项

    // 测试模块现在使用 Solution 中的辅助函数

    #[test]
    fn test_module_concepts() {
        // 演示模块系统的概念

        // 1. 创建链表节点
        let single_node = Some(Box::new(ListNode::new(5)));

        // 2. 使用helper_functions模块中的函数
        let debug_info = format_list_debug(&single_node);
        println!("调试信息: {}", debug_info);

        // 3. 演示不同的访问方式
        let list = Solution::create_list(vec![2, 4, 3]);

        // 测试转换函数
        assert_eq!(
            Solution::list_to_vec(Solution::create_list(vec![2, 4, 3])),
            vec![2, 4, 3]
        );
        assert_eq!(Solution::list_to_vec(None), vec![]);

        // 4. 演示作用域 - 这些变量只在这个测试函数中可见
        let _empty_list: Option<Box<ListNode>> = None;
    }

    #[test]
    fn test_module_visibility() {
        // 演示模块可见性规则

        // ✅ 可以访问：父模块的pub项 (ListNode, Solution)
        let node = ListNode::new(42);

        assert_eq!(node.val, 42);

        // ✅ 可以访问：Solution 中的公共函数
        let list = Solution::create_list(vec![1, 2, 3]);
        assert_eq!(Solution::list_to_vec(list), vec![1, 2, 3]);

        // ✅ 可以访问：helper_functions模块的pub函数
        let empty = None;
        let debug = format_list_debug(&empty);
        assert_eq!(debug, "None");
    }

    #[test]
    fn test_add_two_numbers_basic() {
        // 测试用例: [2,4,3] + [5,6,4] = [7,0,8]
        // 表示: 342 + 465 = 807
        let l1 = Solution::create_list(vec![2, 4, 3]);
        let l2 = Solution::create_list(vec![5, 6, 4]);

        let result = Solution::add_two_numbers(l1, l2);
        let expected = vec![7, 0, 8];

        assert_eq!(Solution::list_to_vec(result), expected);
    }
}
