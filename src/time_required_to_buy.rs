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
