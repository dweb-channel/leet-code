#[cfg(test)]
mod test {
    use std::i32;

    #[test]
    fn test_3171() {
        assert_eq!(0, minimum_difference(vec![1, 2, 4, 5], 3));
        assert_eq!(1, minimum_difference(vec![1, 3, 1, 3], 2));
        assert_eq!(4, minimum_difference(vec![1, 10], 4));
    }
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut min_diff = (nums[0] ^ k).abs(); // 最小的绝对差值初始化为第一个元素与k的差值
        for i in 0..nums.len() {
            let x = nums[i];
            min_diff = min_diff.min((x - k).abs()); // 单个元素也算子数组
            let mut j = i - 1;
            //  如果 x 是 nums[j] 的子集，就退出循环
            while j < nums.len() && (nums[j] | x) != nums[j] {
                nums[j] |= x;
                min_diff = min_diff.min((nums[j] - k).abs());
                j -= 1;
            }
        }
        min_diff
    }
}
