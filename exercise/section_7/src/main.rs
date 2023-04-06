use rand::seq::SliceRandom;
use rand::thread_rng;

use std::collections::BinaryHeap;
fn main() {
    let mut nums: Vec<i32> = vec![];

    nums.push(1);
    nums.push(2);
    nums.push(3);
    nums.push(4);

    let pop = nums.pop();
    println!("The popped value is: {:?}", pop.unwrap());

    let two = nums[1];
    println!("{}", two);

    let one = nums.last();
    println!("{:?}", one);
    println!("check if vec is empty{}", nums.is_empty());
    println!("length of vec {}", nums.len());
    nums.insert(0, 0);
    nums.insert(3, 5);
    nums.insert(4, 2);
    println!("after insert {:?}", nums);
    nums.sort();
    println!("Sorted nums vec: {:?}", nums);
    nums.reverse();
    println!("Reversed nums vec: {:?}", nums);

    nums.shuffle(&mut thread_rng());
    println!("Shuffled: {:?}", nums);

    let mut bheap = BinaryHeap::new();
    bheap.push(10);
    bheap.push(13);
    bheap.push(23);
    bheap.push(14);
    bheap.push(18);

    println!("Binary heap {:?}", bheap);
    bheap.pop();
    println!("Binary heap {:?}", bheap);
    println!("peek: {:?}", bheap.peek());
    println!("Binary heap {:?}", bheap);
}
