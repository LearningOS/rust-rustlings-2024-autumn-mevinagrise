// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.


use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // 使用 Arc 和 Mutex 包裹 JobStatus，以便多个线程可以共享并安全地修改
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];

    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250)); // 睡眠前不要持有锁

            // 锁定 Mutex 以安全地更新 jobs_completed
            let mut status = status_shared.lock().unwrap();
            status.jobs_completed += 1; // 更新共享状态
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap(); // 等待所有线程完成
    }

    // 打印最终的 jobs_completed 值
    let final_status = status.lock().unwrap(); // 锁定以读取值
    println!("jobs completed {}", final_status.jobs_completed);
}
