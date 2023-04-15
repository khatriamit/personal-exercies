use std::thread;
// use std::time::Duration;

fn main() {
    let handle = thread::spawn(move || println!("Hello from a thread!"));
    // thread::sleep(Duration::from_secs(1));

    handle.join().unwrap();
    println!("Hello from main");

    let v = vec![1, 2, 3];
    let handle1 = std::thread::spawn(move || {
        println!("{:?}", v);
    });
    let v1 = vec![1, 2, 3];
    let mut thread_handler = Vec::new();
    for e in v1 {
        thread_handler.push(thread::spawn(move || println!("Thread {}", e)));
    }
    println!("Main Thread!");
    for handle in thread_handler {
        handle.join().unwrap();
    }
}
