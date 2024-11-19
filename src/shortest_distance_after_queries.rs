#[cfg(test)]
mod test {

  #[test] 
  fn test() {
    assert_eq!(vec![3,2,1],shortest_distance_after_queries(5, vec![vec![2,4],vec![0,2],vec![0,4]]));
  }

  pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut prev: Vec<Vec<i32>> = vec![Vec::new(); n as usize]; // 记录通往城市 i 的所有单向道路的起始城市集合
    let mut dp: Vec<i32> = vec![0; n as usize]; // 表示城市 0 到城市 i 的最短路径
    for i in 1..n {
        prev[i as usize].push(i - 1);
        dp[i as usize] = i;
    }
    let mut res: Vec<i32> = Vec::new();
    for query in queries {
        prev[query[1] as usize].push(query[0]);
        for v in query[1] as usize..n as usize {
            for &u in &prev[v] {
                dp[v] = dp[v].min(dp[u as usize] + 1); // dp[i]=mindp[j]+1
            }
        }
        res.push(dp[(n - 1) as usize]);
    }
    res
  }

}