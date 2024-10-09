// 1845
#[cfg(test)]
mod test {

    #[test]
    fn test() {
        let mut obj = SeatManager::new(5);
        let ret_1: i32 = obj.reserve();
        println!("ret_1:{}", ret_1);
        obj.unreserve(2);
    }
    struct SeatManager {
        available_seats: BinaryHeap<Reverse<i32>>,
    }

    use std::cmp::Reverse;
    /**
     * `&self` means the method takes an immutable reference.
     * If you need a mutable reference, change it to `&mut self` instead.
     */
    use std::collections::BinaryHeap;

    impl SeatManager {
        fn new(n: i32) -> Self {
            let mut available_seats = BinaryHeap::with_capacity(n as usize);
            for i in 1..=n {
                available_seats.push(Reverse(i));
            }
            SeatManager { available_seats }
        }

        fn reserve(&mut self) -> i32 {
            if let Some(Reverse(seat)) = self.available_seats.pop() {
                seat
            } else {
                -1
            }
        }

        fn unreserve(&mut self, seat_number: i32) {
            self.available_seats.push(Reverse(seat_number));
        }
    }
}
