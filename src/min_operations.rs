#[cfg(test)]
mod test {
    #[test]
    fn test_3191() {
        assert_eq!(3, min_operations(vec![0, 1, 1, 1, 0, 0]));
    }

    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans = 0;
        for i in 0..n {
            if nums[i] == 0 {
                if i > n - 3 {
                    return -1;
                }
                nums[i] ^= 1;
                nums[i + 1] ^= 1;
                nums[i + 2] ^= 1;
                ans += 1;
            }
        }
        ans
    }

    #[test]
    fn test_3192() {
        assert_eq!(4, min_operations_2(vec![0, 1, 1, 0, 1]));
    }

    pub fn min_operations_2(nums: Vec<i32>) -> i32 {
        let mut operation = 0;
        for num in nums {
            if num == (operation % 2) {
                operation += 1;
            }
        }
        operation
        // let n = nums.len();
        // let mut ans = 0;
        // loop {
        //     let mut zero = (0, n);
        //     let mut one = (0, n);
        //     // 反向找到最多的元素的第一个，重复此步骤
        //     for i in n..0 {
        //         if nums[i] == 0 {
        //             zero.0 += 1;
        //             zero.1 = i;
        //         } else {
        //             one.0 += 1;
        //             one.1 = i;
        //         }
        //     }

        //     let index = if zero.0 > one.0 { zero.1 } else { one.1 };
        //     for i in index..n {
        //         if nums[i] == 0 {
        //             nums[i] ^= 1;
        //         }
        //     }
        //     ans += 1;
        //     let size_num = nums.clone().into_iter().filter(|x| *x == 0).count();
        //     if size_num == 0 {
        //         return ans;
        //     }
        // }
    }
}
