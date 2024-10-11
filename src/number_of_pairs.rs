#[cfg(test)]
mod test {
    use std::{collections::HashMap, i32};

    #[test]
    fn test_3164() {
        assert_eq!(5, number_of_pairs(vec![1, 3, 4], vec![1, 3, 4], 1));
        // assert_eq!(2, number_of_pairs(vec![1, 2, 4, 12], vec![2, 4], 3));
        // assert_eq!(0, number_of_pairs(vec![4, 10], vec![1, 7], 3));
        // assert_eq!(2, number_of_pairs(vec![28, 42], vec![6, 4], 7));
    }

    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let mut cnt1 = HashMap::new();
        // 将所有因子统计到hash表
        for x in nums1 {
            if x % k == 0 {
                *cnt1.entry(x / k).or_insert(0) += 1;
            }
        }
        if cnt1.is_empty() {
            return 0;
        }

        let mut cnt2 = HashMap::new();
        for x in nums2 {
            *cnt2.entry(x).or_insert(0) += 1;
        }

        let mut ans = 0i64;
        let u = *cnt1.keys().max().unwrap();
        for (x, cnt) in cnt2 {
            let mut s = 0;
            for y in (x..=u).step_by(x as usize) {
                // 枚举 x 的倍数 如果命中了直接加上去
                if let Some(&c) = cnt1.get(&y) {
                    s += c;
                }
            }
            ans += s as i64 * cnt as i64;
        }
        ans
    }
}
