mod solutions {
    pub mod two_sum;
}


fn main() {
    // 示例：两数之和
    let nums: Vec<i32> = vec![2, 7, 11, 15];
    let target = 9;
    let result = solutions::two_sum::Solution::two_sum(nums, target);
    println!("two_sum result: {:?}", result);
}
