use std::{sync::mpsc::channel, thread, time};

use threadpool::ThreadPool;

fn job(num: i32) -> (i32, i32) {
    println!("job{} started", num);
    thread::sleep(time::Duration::from_secs(3));
    println!("job{} finished", num);
    (num, num * num)
}

fn main() {
    let pool = ThreadPool::new(5);

    let (tx, rx) = channel();

    for i in 0..15 {
        let tx = tx.clone();
        pool.execute(move || {
            tx.send(job(i)).unwrap();
        })
    }
    drop(tx);

    let res: Vec<_> = rx.iter().collect();
    dbg!(res);
}
