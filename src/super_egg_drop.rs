#[cfg(test)]
mod test {
    use std::collections::HashMap;

    #[test]
    fn test_887() {
        assert_eq!(2, super_egg_drop(1, 2));
    }

    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        let mut res_map: HashMap<(i32, i32), i32> = HashMap::with_capacity(n.try_into().unwrap());
        fn dp(k: i32, n: i32, res_map: &mut HashMap<(i32, i32), i32>) -> i32 {
            if k == 1 {
                return n;
            }
            if n == 0 {
                return 0;
            }
            // 避免重复计算
            if let Some(&result) = res_map.get(&(k, n)) {
                return result;
            }

            let mut res = i32::MAX;
            let mut l = 1;
            let mut r = n;

            // 二分搜索，优化计算次数
            while l <= r {
                let mid = (l + r) / 2;
                let broken = dp(k - 1, mid - 1, res_map); // 鸡蛋碎了
                let not_broken = dp(k, n - mid, res_map); // 鸡蛋没碎
                let worst_case = broken.max(not_broken) + 1;

                if broken > not_broken {
                    r = mid - 1; // 向左查找
                } else {
                    l = mid + 1; // 向右查找
                }

                res = res.min(worst_case);
            }

            // 结果存入哈希表中，避免重复计算
            res_map.insert((k, n), res);
            return res;
        }

        dp(k, n, &mut res_map)
    }
}
