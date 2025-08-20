mod solutions {
    pub mod two_sum;
}

fn main() {
    println!("🚀 Rust 学习演示程序");
    println!("=====================================\n");
    
    println!("\n{}", "=".repeat(50));
    
    // 原有的 LeetCode 示例
    println!("\n【第三部分：LeetCode 示例 - 两数之和】");
    println!("=====================================");
    let nums: Vec<i32> = vec![2, 7, 11, 15];
    let target = 9;
    println!("输入数组: {:?}", nums);
    println!("目标值: {}", target);
    let result = solutions::two_sum::Solution::two_sum(nums, target);
    println!("two_sum 结果: {:?}", result);
    
    println!("\n✅ 所有演示完成！");
}
