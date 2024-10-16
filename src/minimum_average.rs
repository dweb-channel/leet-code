#[cfg(test)]
mod test {
    #[test]
    fn test_3194() {
        assert_eq!(5.5, minimum_average(vec![7, 8, 3, 4, 15, 13, 4, 1]));
    }
    pub fn minimum_average(mut nums: Vec<i32>) -> f64 {
        nums.sort(); // 先排序
        let mut min = f64::MAX;
        let (mut i, mut j) = (0, nums.len() - 1);
        while i < j {
            let cut = (nums[i] as f64 + nums[j] as f64) / 2.0;
            if cut < min {
                min = cut;
            }
            i += 1;
            j -= 1;
        }
        return min;
    }
}
