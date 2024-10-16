// mod max_height_of_triangle;
mod minimum_average;
mod minimum_difference;
mod number_of_pairs;
mod remaining_methods;
mod seat_manager;
mod super_egg_drop;
mod time_required_to_buy;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    #[test]
    fn test_1436() {
        let result = dest_city(vec![
            vec!["London".to_string(), "New York".to_string()],
            vec!["New York".to_string(), "Lima".to_string()],
            vec!["Lima".to_string(), "Sao Paulo".to_string()],
        ]);
        assert_eq!("Sao Paulo".to_string(), result);
    }
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut hash: HashSet<String> = HashSet::with_capacity(paths.len() * 2);
        for path in &paths {
            hash.insert(path[0].clone());
        }
        for path in &paths {
            if !hash.contains(&path[1]) {
                return path[1].clone();
            }
        }
        String::from("")
    }
}
