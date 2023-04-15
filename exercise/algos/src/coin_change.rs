pub fn coin_change(coins: &[u32], amount: u32) -> i32 {
    let mut dp = vec![None, amount + 1];
    dp[0] = Some(0);
    for i in 1..=amount {
        for j in 0..coins.len() {
            if coins[j] <= i {
                dp[i as usize] = dp[i as usize].min(dp[(i - coins[j]) as usize] + 1);
            }
        }
    }
    if dp[amount as usize] > amount {
        return -1;
    }
    dp[amount as usize] as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        // 11 = 5 * 2 + 1 * 1
        let coins = vec![1, 2, 5];
        assert_eq!(3, coin_change(&coins, 11));

        // 119 = 11 * 10 + 7 * 1 + 2 * 1
        let coins = vec![2, 3, 5, 7, 11];
        assert_eq!(12, coin_change(&coins, 119));
    }

    #[test]
    fn coins_empty() {
        let coins = vec![];
        assert_eq!(0, coin_change(&coins, 1));
    }

    #[test]
    fn amount_zero() {
        let coins = vec![1, 2, 3];
        assert_eq!(0, coin_change(&coins, 0));
    }

    #[test]
    fn fail_change() {
        // 3 can't be change by 2.
        let coins = vec![2];
        assert_eq!(0, coin_change(&coins, 3));
        let coins = vec![10, 20, 50, 100];
        assert_eq!(0, coin_change(&coins, 5));
    }
}
