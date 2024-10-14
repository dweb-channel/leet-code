#[cfg(test)]
mod test {
    use std::collections::HashSet;

    #[test]
    fn test_2703() {
        assert_eq!(0, time_required_to_buy(vec![1, 2, 4, 5], 3));
    }
    // 2703
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let target = tickets.get(k as usize).unwrap();
        let mut result: i32 = 0;
        for (i, ticket) in tickets.iter().enumerate() {
            if ticket > target {
                result += target;
                if i > k.try_into().unwrap() {
                    result -= 1;
                }
            } else {
                result += target;
            }
        }
        return result;
    }

    #[test]
    fn test_3158() {
        assert_eq!(duplicate_numbers_xor(vec![1, 2, 1, 3]), 1);
    }

    pub fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
        let mut cnt = HashSet::new();
        let mut res = 0;

        for num in nums {
            if cnt.contains(&num) {
                res ^= num;
            } else {
                cnt.insert(num);
            }
        }
        res
    }
}
