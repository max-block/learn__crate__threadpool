use std::{sync::mpsc::channel, thread, time};

use threadpool::ThreadPool;

fn job(num: i32) {
    println!("job{} started", num);
    thread::sleep(time::Duration::from_secs(3));
    println!("job{} finished", num);
}

fn main() {
    let pool = ThreadPool::new(5);

    let (tx, rx) = channel();

    for _ in 0..15 {
        let tx = tx.clone();
        pool.execute()

    }
}
