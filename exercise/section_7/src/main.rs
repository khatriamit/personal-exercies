use rand::seq::SliceRandom;
use rand::thread_rng;

use std::collections::{BinaryHeap, HashMap, HashSet};
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

    let mut hm = HashMap::new();
    hm.insert(1, 1);
    hm.insert(5, 2);
    hm.insert(30, 3);
    let old = hm.insert(30, 4); // here the key's value is updated and old value is assigned to new variable

    println!("HashMap: {:?}", hm);
    println!("Value: {:?}", old);

    println!("{}", hm.contains_key(&6));
    println!("{:?}", hm.get(&5));
    hm.remove(&1);
    println!("HashMap: {:?}", hm);

    let mut hs1 = HashSet::new();
    hs1.insert(1);
    hs1.insert(2);
    hs1.insert(3);

    for x in hs1.intersection(&hs1) {
        println!("Intersection {}", x);
    }

    let mut hs2 = HashSet::new();
    hs2.insert(1);
    hs2.insert(2);
    hs2.insert(3);
    hs2.insert(5);

    for x in hs2.iter() {
        println!("Iter: {}", x);
    }
    let uni = &hs2 | &hs2;
    for x in uni {
        println!("Union: {}", x);
    }
    let diff = &hs2 - &hs1;
    for x in diff {
        println!("Difference: {}", x);
    }
}
