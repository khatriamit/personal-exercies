mod coin_change;
fn main() {
    let coins = vec![1, 2, 5];
    let coin = coin_change::coin_change(&coins, 12);
    println!("The ans is {}", coin);
}
