use std::cmp::max;
#[warn(dead_code)]
pub fn max_height_of_triangle(mut red: i32, mut blue: i32) -> i32 {
    fn max_height(mut x: i32, mut y: i32) -> i32 {
        // 第一行有一个
        let mut i = 1;
        loop {
            // 偶数奇数交替排
            if i % 2 == 1 {
                x -= i;
                if x < 0 {
                    return i - 1;
                }
            } else {
                // 消耗掉i个球
                y -= i;
                // 如果排不下去了，返回上一层
                if y < 0 {
                    return i - 1;
                }
            }
            // 第二行两个，每循环一次加一
            i += 1;
        }
    }
    max(max_height(red, blue), max_height(blue, red))
}
